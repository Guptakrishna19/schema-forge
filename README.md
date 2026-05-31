# SchemaForge

A **metadata-driven application platform**: define entities, fields, relations,
permissions, and workflows as schema; the platform generates the runtime — APIs,
forms, tables, validation, permissions, real-time, and workflow execution — with
**no entity-specific code**.

## Repository layout

| Path | What |
|---|---|
| [`docs/`](./docs) | PRD, system design, design plan, execution plan, tracker (source of truth) |
| [`backend/`](./backend) | Rust / Axum backend — schema engine, CRUD, permissions, events, discovery |
| [`frontend/`](./frontend) | React / TypeScript schema-driven UI (generic form + table) |
| `docker-compose.yml` | Local Postgres + Apache Kafka |

## Stack

- **Backend:** Rust (Tokio) + Axum, PostgreSQL + JSONB via SQLx
- **Events:** Apache Kafka (rdkafka), fed by a transactional DB outbox
- **Frontend:** React + TypeScript (Vite)

## Quick start

```bash
# 1. Dependencies
docker compose up -d

# 2. Backend (requires the Rust toolchain — see backend/README.md)
cd backend && cp .env.example .env && cargo run        # :8080

# 3. Frontend
cd frontend && pnpm install && pnpm dev                 # :5173

# 4. Activate the reference schema, then open the UI
curl -X POST localhost:8080/api/admin/schema/versions/v1/activate \
  -H 'content-type: application/json' \
  --data @backend/fixtures/customer-order.json
```

## Status

Greenfield scaffold. Components for not-yet-built phases return `501 Not
Implemented` with a `TODO(Pn)` marker pointing at the delivering phase in
[`docs/execution-plan.md`](./docs/execution-plan.md). Live status:
[`docs/tracker.md`](./docs/tracker.md).
