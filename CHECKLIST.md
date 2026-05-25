# Checklist — Docker Compose

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

---

## Usługi i porty

| Usługa   | Obraz / Build           | Port wewnętrzny | Port zewnętrzny | Opis                          |
|----------|-------------------------|-----------------|-----------------|-------------------------------|
| db       | postgres:16-alpine      | 5432            | —               | PostgreSQL, dane na wolumenie |
| backend  | ./backend (Rust/Axum)   | 3000            | —               | REST API + WebSocket          |
| frontend | ./frontend (Angular)    | 80              | —               | SPA serwowana przez nginx     |
| nginx    | nginx:alpine            | 80              | **80**          | Reverse proxy                 |

Tylko `nginx` jest dostępny z zewnątrz. Pozostałe usługi komunikują się wewnątrz sieci `internal`.

---

## Zasoby Docker

- **Obrazy**: `postgres:16-alpine`, `nginx:alpine`, `rust:slim-bookworm` (build), `debian:bookworm-slim` (runtime), `node:20-alpine` (build), `nginx:alpine` (runtime)
- **Sieć**: `internal` (bridge) — izoluje usługi od zewnątrz
- **Wolumen**: `db_data` — trwałe przechowywanie danych PostgreSQL
- **Healthchecks**: db (pg_isready), backend (GET /health), nginx (wget /)
- **Sekrety**: hasło do bazy jako Docker secret (`secrets/db_password.txt` → `/run/secrets/db_password`)

---

## Komendy testowe

### Sprawdzenie statusu

```bash
# Status wszystkich kontenerów
docker compose ps

# Logi wybranej usługi
docker compose logs backend
docker compose logs db
docker compose logs nginx
```

### API backendu

```bash
# Health check
curl http://localhost/health

# Utwórz nowy czat
curl -X POST http://localhost/api/chats/ \
  -H "Content-Type: application/json" \
  -d '{"title": "testowy-czat"}'

# Pobierz wiadomości
curl http://localhost/api/chats/testowy-czat/messages
```

### WebSocket (wymaga wscat: npm install -g wscat)

```bash
wscat -c "ws://localhost/api/chats/connect?username=jan&chat_name=testowy-czat"
# W połączeniu wpisz:
# {"content": "Cześć!", "author_name": "jan"}
```

### Baza danych

```bash
docker compose exec db psql -U postgres -d chat -c "\dt"
docker compose exec db psql -U postgres -d chat -c "SELECT * FROM chats;"
docker compose exec db psql -U postgres -d chat -c "SELECT * FROM messages;"
```

---

## Przykładowe wyniki

### `docker compose ps`
```
NAME                    IMAGE             STATUS                   PORTS
project-db-1            postgres:16-alpine   Up (healthy)
project-backend-1       project-backend      Up (healthy)
project-frontend-1      project-frontend     Up
project-nginx-1         nginx:alpine         Up (healthy)          0.0.0.0:80->80/tcp
```

### `curl http://localhost/health`
```
HTTP/1.1 200 OK
```

### Tworzenie czatu
```json
{"id":1,"title":"testowy-czat","created_at":"2026-05-25T12:00:00Z"}
```

---

## Wymagania dodatkowe — status

| Wymaganie                        | Status |
|----------------------------------|--------|
| Dockerfile dla każdej usługi     | ✅     |
| Docker Compose                   | ✅     |
| Sieć wewnętrzna (bridge)         | ✅     |
| Trwały wolumen dla bazy danych   | ✅     |
| Healthchecks                     | ✅     |
| Reverse proxy (nginx)            | ✅     |
| Docker secrets (hasło do DB)     | ✅     |
| WebSocket przez proxy            | ✅     |
| Multi-stage builds               | ✅     |
| Restart policy                   | ✅     |

---

## Zatrzymanie i czyszczenie

```bash
# Zatrzymaj usługi (dane zostają)
docker compose down

# Zatrzymaj i usuń wolumeny (reset bazy danych)
docker compose down -v

# Przebuduj konkretną usługę
docker compose build backend
docker compose up -d backend
```
