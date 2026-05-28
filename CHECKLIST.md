# Checklist — Docker Compose & Kubernetes

---

# Część 1 — Docker Compose

## Uruchomienie od zera

```bash
# 1. Sklonuj repozytorium
git clone <repo-url> && cd ug-chmurowe-project

# 2. Przygotuj zmienne środowiskowe
cp .env.example .env

# 3. Utwórz plik z hasłem do bazy danych (Docker secret)
mkdir -p secrets
echo "changeme" > secrets/db_password.txt

# 4. Zbuduj obrazy i uruchom wszystkie usługi
docker compose up --build -d

# 5. Sprawdź status usług (wszystkie powinny być healthy)
docker compose ps

# 6. Otwórz aplikację
open http://localhost   # lub wpisz ręcznie w przeglądarce
```

> Pierwsze uruchomienie może zająć kilka minut — backend (Rust) kompiluje się od zera.  
> Baza danych i migracje uruchamiają się automatycznie.

## Usługi i porty

| Usługa   | Obraz / Build           | Port wewnętrzny | Port zewnętrzny | Opis                          |
|----------|-------------------------|-----------------|-----------------|-------------------------------|
| db       | postgres:16-alpine      | 5432            | —               | PostgreSQL, dane na wolumenie |
| backend  | ./backend (Rust/Axum)   | 3000            | —               | REST API + WebSocket          |
| frontend | ./frontend (Angular)    | 80              | —               | SPA serwowana przez nginx     |
| nginx    | nginx:alpine            | 80              | **80**          | Reverse proxy                 |

Tylko `nginx` jest dostępny z zewnątrz. Pozostałe usługi komunikują się wewnątrz sieci `internal`.

## Zasoby Docker

- **Obrazy**: `postgres:16-alpine`, `nginx:alpine`, `rust:slim-bookworm` (build), `debian:bookworm-slim` (runtime), `node:20-alpine` (build), `nginx:alpine` (runtime)
- **Sieć**: `internal` (bridge) — izoluje usługi od zewnątrz
- **Wolumen**: `db_data` — trwałe przechowywanie danych PostgreSQL
- **Healthchecks**: db (pg_isready), backend (GET /health), nginx (wget /)
- **Sekrety**: hasło do bazy jako Docker secret (`secrets/db_password.txt` → `/run/secrets/db_password`)

## Komendy testowe (Compose)

```bash
# Status
docker compose ps
docker compose logs backend

# Health check
curl http://localhost/api/health

# Utwórz czat
curl -X POST http://localhost/api/chats/ \
  -H "Content-Type: application/json" \
  -d '{"title": "testowy-czat"}'

# Pobierz wiadomości
curl http://localhost/api/chats/testowy-czat/messages

# WebSocket (npm install -g wscat)
wscat -c "ws://localhost/api/chats/connect?username=jan&chat_name=testowy-czat"

# Baza danych
docker compose exec db psql -U postgres -d chat -c "SELECT * FROM chats;"
```

## Zatrzymanie

```bash
docker compose down        # dane zostają
docker compose down -v     # reset bazy
```

---

# Część 2 — Kubernetes

## Wymagania wstępne

Klaster musi być uruchomiony **przed** pipeline CI/CD. To jednorazowa konfiguracja środowiska.

```bash
# Utwórz klaster z mapowaniem portu 80
kind create cluster --name chat --config kind-config.yaml

# Zainstaluj nginx ingress controller
kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/kind/deploy.yaml
kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s
```

Lub jedną komendą: `make k8s-cluster k8s-ingress`

## Przygotowanie obrazów

```bash
# Zbuduj i załaduj obrazy do klastra kind
docker build -t chat-backend:latest ./backend
docker build -t chat-frontend:latest ./frontend
kind load docker-image chat-backend:latest --name chat
kind load docker-image chat-frontend:latest --name chat
```

## Uruchomienie klastra

```bash
# Zastosuj wszystkie manifesty
kubectl apply -f k8s/ --dry-run=client   # walidacja
kubectl apply -f k8s/

# Sprawdź status podów (poczekaj aż wszystkie Running)
kubectl get pods -w

# Sprawdź ingress
kubectl get ingress
```

## Zasoby Kubernetes

| Zasób                   | Nazwa           | Opis                                    |
|-------------------------|-----------------|-----------------------------------------|
| Deployment              | postgres        | Baza danych PostgreSQL                  |
| Deployment              | backend         | Rust/Axum API                           |
| Deployment              | frontend        | Angular SPA                             |
| Service (ClusterIP)     | postgres        | Wewnętrzny dostęp do DB na port 5432    |
| Service (ClusterIP)     | backend         | Wewnętrzny dostęp do API na port 3000   |
| Service (ClusterIP)     | frontend        | Wewnętrzny dostęp do frontu na port 80  |
| PersistentVolumeClaim   | postgres-pvc    | Trwały dysk 1Gi dla danych PostgreSQL   |
| ConfigMap               | app-config      | Zmienne środowiskowe (user, db, host)   |
| Secret                  | db-password     | Hasło do bazy (base64)                  |
| Ingress                 | app-ingress     | Routing: /api → backend, / → frontend  |

## Komendy kubectl

```bash
# Status podów i deploymentów
kubectl get pods
kubectl get deployments
kubectl get services
kubectl get ingress
kubectl get pvc

# Logi
kubectl logs deployment/backend
kubectl logs deployment/postgres

# Exec do kontenera
kubectl exec -it deployment/postgres -- psql -U postgres -d chat

# Opisz zasób (debug)
kubectl describe pod <nazwa-poda>
kubectl describe ingress app-ingress
```

## Przykładowe wyniki

### `kubectl get pods`
```
NAME                        READY   STATUS    RESTARTS   AGE
backend-7d6b9f8c4-xk2p9     1/1     Running   0          2m
frontend-5c8d4b7f6-mn3q1    1/1     Running   0          2m
postgres-6f9c5d8b7-pw4r2    1/1     Running   0          3m
```

### `kubectl get ingress`
```
NAME          CLASS   HOSTS   ADDRESS     PORTS   AGE
app-ingress   nginx   *       localhost   80      2m
```

## CI/CD — GitHub Actions

Pipeline uruchamia się przy każdym push na `master` na **self-hosted runnerze** (działa lokalnie na tej samej maszynie co klaster kind):

1. Waliduje manifesty (`kubectl apply --dry-run=client -f k8s/`)
2. Buduje obrazy Docker lokalnie
3. Ładuje obrazy do klastra kind (`kind load docker-image`)
4. Aplikuje manifesty i czeka na rollout

**Uruchomienie runnera (jednorazowo):**
```bash
cd actions-runner/actions-runner
./config.sh --url https://github.com/imizgun/ug-chmurowe-project --token <TOKEN>
./run.sh
```
Token jednorazowy: Settings → Actions → Runners → New self-hosted runner.

Link do ostatniego workflow: <https://github.com/imizgun/ug-chmurowe-project/actions>

## Wymagania dodatkowe — Kubernetes

| Wymaganie                        | Status |
|----------------------------------|--------|
| Manifesty Deployment/Service     | ✅     |
| PersistentVolumeClaim (postgres) | ✅     |
| Ingress (nginx)                  | ✅     |
| Liveness i Readiness probes      | ✅     |
| Resource requests i limits       | ✅     |
| securityContext                  | ✅     |
| Secret (hasło do DB jako plik)   | ✅     |
| ConfigMap                        | ✅     |
| CI/CD — GitHub Actions           | ✅     |
