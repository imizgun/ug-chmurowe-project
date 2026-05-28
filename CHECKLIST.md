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

| Usługa   | Obraz / Build           | Port wewnętrzny | Port zewnętrzny | Opis                              |
|----------|-------------------------|-----------------|-----------------|-----------------------------------|
| db       | postgres:16-alpine      | 5432            | —               | PostgreSQL, dane na wolumenie     |
| backend  | ./backend (Rust/Axum)   | 3000            | —               | REST API + WebSocket              |
| redis    | redis:7-alpine          | 6379            | —               | Cache / pub-sub dla wiadomości    |
| frontend | ./frontend (Angular)    | 80              | —               | SPA serwowana przez nginx         |
| nginx    | nginx:alpine            | 80              | **80**          | Reverse proxy                     |

Tylko `nginx` jest dostępny z zewnątrz. Pozostałe usługi komunikują się wewnątrz izolowanych sieci.

## Architektura sieci

```
browser
   │
   ▼
nginx (frontend-net + backend-net)
   ├── frontend (frontend-net)
   └── backend  (backend-net)
                   ├── db    (db-net)
                   └── redis (backend-net)
```

Trzy izolowane sieci bridge: `frontend-net`, `backend-net`, `db-net`.  
Baza danych nie ma wystawionego portu na hosta.

## Zasoby Docker

- **Obrazy**: `postgres:16-alpine`, `redis:7-alpine`, `nginx:alpine`, `rust:slim-bookworm` (build), `debian:bookworm-slim` (runtime), `node:20-alpine` (build)
- **Sieci**: `frontend-net`, `backend-net`, `db-net` — każda izolowana
- **Wolumen**: `db_data` — trwałe przechowywanie danych PostgreSQL
- **Healthchecks**: db (pg_isready), backend (GET /api/health), redis (redis-cli ping), nginx (wget /)
- **Sekrety**: hasło do bazy jako Docker secret (`secrets/db_password.txt` → `/run/secrets/db_password`)
- **Limity zasobów**: ustawione dla każdej usługi (`deploy.resources.limits`)
- **Rotacja logów**: `json-file`, max-size 10m, max-file 3
- **Graceful shutdown**: `stop_grace_period: 30s` dla backendu

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

# Sprawdź pub/sub Redis (w osobnym terminalu przed wysłaniem wiadomości)
docker compose exec redis redis-cli subscribe chat:1

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
# Zastosuj wszystkie manifesty (namespace tworzony automatycznie)
kubectl apply -f k8s/ --dry-run=client   # walidacja
kubectl apply -f k8s/

# Sprawdź status podów w namespace chat
kubectl get pods -n chat -w

# Sprawdź ingress
kubectl get ingress -n chat
```

## Zasoby Kubernetes

| Zasób                   | Nazwa           | Namespace | Opis                                         |
|-------------------------|-----------------|-----------|----------------------------------------------|
| Namespace               | chat            | —         | Izolacja zasobów projektu                    |
| StatefulSet             | db              | chat      | PostgreSQL z własnym PVC (volumeClaimTemplate)|
| Deployment              | backend         | chat      | Rust/Axum API, 2 repliki, RollingUpdate      |
| Deployment              | frontend        | chat      | Angular SPA                                  |
| Deployment              | redis           | chat      | Cache / pub-sub                              |
| Service (headless)      | db              | chat      | Wewnętrzny dostęp do DB na port 5432         |
| Service (ClusterIP)     | backend         | chat      | Wewnętrzny dostęp do API na port 3000        |
| Service (ClusterIP)     | frontend        | chat      | Wewnętrzny dostęp do frontu na port 80       |
| Service (ClusterIP)     | redis           | chat      | Wewnętrzny dostęp do Redis na port 6379      |
| ConfigMap               | app-config      | chat      | Zmienne środowiskowe (user, db, host, redis) |
| Secret                  | db-password     | chat      | Hasło do bazy (base64)                       |
| Ingress                 | app-ingress     | chat      | Routing: /api → backend, / → frontend        |

## Komendy kubectl

```bash
# Status podów i deploymentów
kubectl get pods -n chat
kubectl get deployments -n chat
kubectl get statefulsets -n chat
kubectl get services -n chat
kubectl get ingress -n chat
kubectl get pvc -n chat

# Logi
kubectl logs deployment/backend -n chat
kubectl logs statefulset/db -n chat

# Exec do kontenera
kubectl exec -it statefulset/db -n chat -- psql -U postgres -d chat

# Rolling update status
kubectl rollout status deployment/backend -n chat

# Opisz zasób (debug)
kubectl describe pod <nazwa-poda> -n chat
kubectl describe ingress app-ingress -n chat
```

## Przykładowe wyniki

### `kubectl get pods -n chat`
```
NAME                        READY   STATUS    RESTARTS   AGE
backend-7d6b9f8c4-xk2p9     1/1     Running   0          2m
backend-7d6b9f8c4-ab3c2     1/1     Running   0          2m
frontend-5c8d4b7f6-mn3q1    1/1     Running   0          2m
redis-6f4d2b8c9-qr5s7       1/1     Running   0          2m
db-0                        1/1     Running   0          3m
```

### `kubectl get ingress -n chat`
```
NAME          CLASS   HOSTS   ADDRESS     PORTS   AGE
app-ingress   nginx   *       localhost   80      2m
```

### `kubectl rollout status deployment/backend -n chat`
```
deployment "backend" successfully rolled out
```

## CI/CD — GitHub Actions

Pipeline uruchamia się przy każdym push na `master` na **self-hosted runnerze**.  
Obrazy są tagowane `latest` oraz SHA commita dla pełnej identyfikowalności.

1. Buduje obrazy z tagiem `latest` i `<git-sha>`
2. Waliduje manifesty (`kubectl apply --dry-run=client -f k8s/`)
3. Ładuje obrazy do klastra kind
4. Aplikuje manifesty w namespace `chat`
5. Czeka na rollout backendu i frontendu

**Uruchomienie runnera (jednorazowo):**
```bash
cd actions-runner/actions-runner
./config.sh --url https://github.com/imizgun/ug-chmurowe-project --token <TOKEN>
./run.sh
```
Token jednorazowy: Settings → Actions → Runners → New self-hosted runner.

Link do ostatniego workflow: <https://github.com/imizgun/ug-chmurowe-project/actions>

## Wymagania — status

| Wymaganie                                  | Status |
|--------------------------------------------|--------|
| Namespace                                  | ✅     |
| StatefulSet (baza danych) + PVC            | ✅     |
| Deployment backend (2 repliki, RollingUpdate) | ✅  |
| Deployment frontend                        | ✅     |
| Deployment redis (cache)                   | ✅     |
| Services (ClusterIP / headless)            | ✅     |
| Ingress (nginx)                            | ✅     |
| ConfigMap                                  | ✅     |
| Secret (hasło do DB)                       | ✅     |
| Liveness i Readiness probes               | ✅     |
| Resource requests i limits                 | ✅     |
| securityContext (non-root, capabilities)   | ✅     |
| initContainer (wait-for-db)               | ✅     |
| CI/CD — GitHub Actions + tagowanie SHA    | ✅     |
