# System Design — SchemaForge

| | |
|---|---|
| **Status** | Draft v1 |
| **Last updated** | 2026-05-30 |
| **Related** | [PRD.md](./PRD.md) · [design-plan.md](./design-plan.md) · [execution-plan.md](./execution-plan.md) |

> Technology choices below are **recommended defaults**, chosen to fit a metadata-driven platform. They are not yet ratified — see [Open decisions](#10-open-decisions).

---

## 1. Design principles

1. **Metadata is the program.** Behavior derives from schema at runtime, not from generated/compiled code.
2. **One generic engine, many entities.** No per-entity classes, routes, or components.
3. **Single source of truth.** DB, API, UI, permissions, and docs all read the same schema.
4. **Deny by default.** Every access decision is explicit; absence of a rule means "no".
5. **Safe by construction.** Workflows and permission rules use a sandboxed expression language, never arbitrary code.
6. **Self-describing.** The system can always emit a complete machine-readable description of itself.

## 2. Architecture overview

```
                         ┌──────────────────────────────────────────┐
   Clients / Agents      │              SchemaForge Core             │
  (web UI, AI agents,    │                                           │
   external services)    │  ┌────────────┐   ┌────────────────────┐ │
        │                │  │  Schema     │   │  Schema Registry   │ │
        │  REST / WS     │  │  Engine     │──▶│  (versioned defs,  │ │
        ├───────────────▶│  │ (validate,  │   │   validation,      │ │
        │                │  │  resolve)   │   │   activation)      │ │
        │                │  └─────┬───────┘   └────────────────────┘ │
        │                │        │                                  │
        │                │  ┌─────▼───────┐   ┌────────────────────┐ │
        │                │  │ Dynamic CRUD│   │  Permission        │ │
        │                │  │ + Query     │──▶│  Evaluator (RBAC/  │ │
        │                │  │ Engine      │   │  ABAC, expr eval)  │ │
        │                │  └─────┬───────┘   └────────────────────┘ │
        │                │        │                                  │
        │                │  ┌─────▼───────┐   ┌────────────────────┐ │
        │                │  │ Persistence │   │  Workflow Engine   │ │
        │                │  │ Adapter     │   │  (triggers,        │ │
        │                │  │ (Postgres)  │   │   conditions,      │ │
        │                │  └─────┬───────┘   │   actions, runs)   │ │
        │                │        │           └─────────┬──────────┘ │
        │                │  ┌─────▼───────────┐         │            │
        │  WS / SSE      │  │ Kafka Event Bus │◀────────┘            │
        │◀───────────────┤  │ + Realtime Hub  │                      │
        │                │  └─────────────────┘                      │
        │                │                                           │
        │  /api/schema   │  ┌─────────────────┐                      │
        │◀───────────────┤  │ Discovery /     │                      │
                         │  │ OpenAPI emitter │                      │
                         │  └─────────────────┘                      │
                         └──────────────────────────────────────────┘
                                          │
                              ┌───────────▼───────────┐
                              │ PostgreSQL (JSONB) +   │
                              │ metadata tables +      │
                              │ event/outbox tables    │
                              └────────────────────────┘
```

## 3. Components

### 3.1 Schema Registry
Stores versioned schema definitions. Responsibilities:
- Persist schema versions (immutable once activated).
- Validate a candidate version (syntax, type integrity, relation targets, permission references).
- Diff against the active version to classify changes as **compatible** (additive) or **breaking**.
- Activate a version atomically; keep history for rollback.

### 3.2 Schema Engine
In-memory resolver that loads the active schema and answers: "what is entity X, its fields, relations, constraints, permission rules, workflows?" Hot-reloads on activation. All other components consume the engine, never raw rows.

### 3.3 Dynamic CRUD & Query Engine
A single set of generic handlers (`create/read/update/delete/list`) parameterized by entity name. It:
- Looks up the entity in the Schema Engine.
- Validates payloads against field constraints.
- Translates list queries (filter/sort/paginate/expand) into safe parameterized SQL.
- Delegates storage to the Persistence Adapter.
- Emits domain events for the Event Bus.

### 3.4 Persistence Adapter
Maps entities to storage. Default strategy (PostgreSQL):
- **Hybrid model** — core/queryable fields as real columns where known; flexible/extension fields in a `data JSONB` column. (See [design-plan.md](./design-plan.md) for the storage-strategy tradeoff.)
- Generates indexes from schema hints (unique constraints, indexed fields, FKs for relations).
- Relations modeled via FK columns (1-1, 1-n) and join tables (n-n), created/migrated by the registry's migration step.

### 3.5 Permission Evaluator
- Resolves role/attribute rules from schema metadata per (entity, action, field).
- Evaluates ABAC expressions (e.g., `record.ownerId == user.id`) in a sandboxed expression evaluator.
- Used by the CRUD engine, the realtime hub (to filter pushes), and the UI (to compute capabilities).
- Deny-by-default; produces field-level read/write masks.

### 3.6 Workflow Engine
- Consumes domain-event Kafka topics (consumer group) and schedule ticks.
- Evaluates declarative triggers → conditions → ordered actions.
- Actions: set field, create/update/delete entity, HTTP webhook, send notification, emit event.
- Uses a transactional **outbox** for reliable, idempotent, retryable execution; records run history.

### 3.7 Event Bus & Realtime Hub
- Domain events (`entity.created/updated/deleted`) written to the transactional outbox on CRUD commit, then relayed to **Apache Kafka** topics.
- Workflow Engine and Realtime Hub are independent Kafka consumers, so each scales and replays independently.
- Realtime Hub fans out to subscribed WebSocket/SSE clients, applying per-client permission filtering.

### 3.8 Discovery / OpenAPI emitter
- `/api/schema` returns the full machine-readable schema.
- Also emits **OpenAPI 3** and **JSON Schema** so agents, codegen, and API explorers self-configure.

## 4. Data model (metadata tables)

```
schema_version (id, version, status[draft|active|archived], created_at, activated_at, definition JSONB)
entity_def      (derived from active schema_version.definition — not a separate table)
audit_log       (id, actor, action, target_type, target_id, before JSONB, after JSONB, at)
workflow_run    (id, workflow_id, trigger, entity, record_id, status, steps JSONB, error, at)
outbox          (id, type, payload JSONB, status, attempts, available_at)
```

Per-entity data (default hybrid strategy):
```
<entity>        (id, <known columns...>, data JSONB, created_at, updated_at, created_by, ...)
<a>_<b>_link    (a_id, b_id)               -- many-to-many join tables
```

## 5. API surface (REST-first)

| Method | Path | Purpose |
|---|---|---|
| GET | `/api/schema` | Full machine-readable schema |
| GET | `/api/openapi.json` | OpenAPI 3 spec |
| GET | `/api/{entity}` | List (filter/sort/paginate/expand) |
| POST | `/api/{entity}` | Create |
| GET | `/api/{entity}/{id}` | Read |
| PATCH | `/api/{entity}/{id}` | Update |
| DELETE | `/api/{entity}/{id}` | Delete |
| WS | `/api/realtime` | Subscribe to entity/collection changes |
| POST | `/api/admin/schema/versions` | Submit candidate schema version |
| POST | `/api/admin/schema/versions/{id}/activate` | Validate + migrate + activate |

List query convention: `?filter[status]=open&sort=-createdAt&page=2&pageSize=50&expand=customer&fields=id,title`.

## 6. Request lifecycle (dynamic CRUD)

1. Router receives `/{entity}` → resolves entity via Schema Engine (404 if unknown).
2. Permission Evaluator checks (entity, action, user) → deny → 403.
3. Validate payload/query against schema constraints → 422 on failure.
4. Build parameterized SQL via Persistence Adapter; apply field-level masks.
5. Execute in a transaction; write `audit_log` and `outbox` event in the same tx.
6. Commit → Event Bus publishes → Workflow Engine + Realtime Hub react.
7. Serialize response honoring read masks and requested field selection.

## 7. Schema change & migration flow

1. Submit candidate version → Registry validates structure, types, relation targets, permission refs.
2. Diff vs active → classify additive (safe) vs breaking (column drop/type narrow/required-add).
3. Additive → activate immediately; adapter runs online DDL (add column/index/join table).
4. Breaking → require explicit migration plan (backfill/transform) before activation; run in a controlled window.
5. Activate atomically; Schema Engine hot-reloads; old version archived for rollback.

## 8. Technology choices (recommended defaults)

| Concern | Choice | Why |
|---|---|---|
| Language/runtime | Rust (stable, async/Tokio) | Memory safety, predictable low-latency performance, strong type system for the generic engine |
| API framework | Axum (Tower/Tokio); Actix-web as alt | Async, modular extractor/middleware model; integrates with the Tokio + rdkafka ecosystem |
| Datastore | PostgreSQL + JSONB | Relational integrity + flexible fields + indexing |
| Query/migrations | SQLx (compile-time-checked SQL) + sqlx migrations | Dynamic parameterized SQL for the generic query builder, migration tooling, no heavy ORM |
| Realtime | WebSocket (tokio-tungstenite) + SSE fallback | Standard, broad client support |
| Event backbone | Apache Kafka (via rdkafka), fed by a transactional DB outbox | Durable, ordered, replayable event log; decouples CRUD from workflow/realtime; scales horizontally |
| Expression sandbox | CEL (cel-interpreter) / JSONLogic-style evaluator | Safe ABAC + workflow conditions, no arbitrary code |
| Frontend | React + TypeScript, schema-driven renderer | Generic form/table components; client types generated from OpenAPI/JSON Schema |
| Spec output | OpenAPI 3 + JSON Schema | AI/tooling interoperability; also generates the TS client types front/back |

## 9. Cross-cutting concerns
- **Security**: deny-by-default, sandboxed expressions, parameterized SQL only, audit log, secret-safe webhooks.
- **Multitenancy**: tenant_id scoping on all rows + row-level filters (decision pending; see below).
- **Observability**: structured logs, request/workflow tracing, metrics per entity and per workflow.
- **Caching**: compiled schema cached in-process; invalidated on activation. Query plans reused.
- **Testing**: contract tests against `/api/schema`; property tests for CRUD across arbitrary schemas.

## 10. Open decisions
- REST-only v1 vs REST+GraphQL.
- Storage strategy: hybrid columns+JSONB (default) vs pure-JSONB vs per-entity physical tables.
- Multitenancy: shared schema + tenant_id vs schema-per-tenant.
- Expression language choice and its capability ceiling.
- Event backbone: **Kafka chosen** (outbox → Kafka). Open: outbox relay via Debezium CDC vs a polling publisher; topic-per-entity vs a shared domain-events topic with keyed partitions.
