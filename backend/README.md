# SchemaForge — Backend (Rust / Axum)

Metadata-driven runtime: versioned schema definitions in, full CRUD + permissions
+ events + discovery out, with no per-entity code. See [`../docs`](../docs) for the
PRD, system design, and execution plan.

## Stack

- **Rust** (async, Tokio) + **Axum** (Tower) HTTP framework
- **PostgreSQL + JSONB** via **SQLx** (compile-time-checked SQL + migrations)
- **Apache Kafka** event backbone via **rdkafka**, fed by a transactional outbox
- **CEL** for the ABAC / workflow expression sandbox (to be wired in P4)

## Module map (→ system-design.md components)

| Module | Component |
|---|---|
| `schema` | Schema Registry & Engine (§3.1–3.2) + meta-schema |
| `persistence` | Persistence Adapter — hybrid columns + JSONB (§3.4) |
| `crud` | Dynamic CRUD & Query Engine (§3.3) |
| `permissions` | Permission Evaluator — RBAC/ABAC (§3.5) |
| `workflow` | Workflow Engine (§3.6) |
| `events` | Outbox → Kafka publisher/relay (§3.7) |
| `realtime` | Realtime Hub — WebSocket/SSE (§3.7) |
| `discovery` | `/api/schema` + OpenAPI emitter (§3.8) |
| `routes` | API surface wiring (§5) |

Handlers for not-yet-built phases return `501 Not Implemented` with a `TODO(Pn)`
marker pointing at the execution-plan phase that delivers them.

## Prerequisites

- Rust toolchain (`rustup`) — `cargo` is not yet installed in this environment.
- For `rdkafka`'s bundled build: **CMake** + a C compiler.
- Postgres and Kafka — use the repo-root `docker-compose.yml`.

## Run

```bash
cp .env.example .env
docker compose -f ../docker-compose.yml up -d   # postgres + kafka
cargo run                                        # serves on :8080
```

The server starts even before Postgres/Kafka are reachable (lazy connections).

## Try it (works in-memory, no DB needed)

```bash
# Activate the reference schema directly into the engine
curl -X POST localhost:8080/api/admin/schema/versions/v1/activate \
  -H 'content-type: application/json' \
  --data @fixtures/customer-order.json

# Discovery reflects it
curl localhost:8080/api/schema

# Known entity resolves (CRUD storage lands in P3 → 501); unknown → 404
curl localhost:8080/api/Customer
curl localhost:8080/api/Nope
```

## Develop

```bash
cargo test          # meta-schema validation tests
cargo clippy --all-targets
cargo fmt
```
