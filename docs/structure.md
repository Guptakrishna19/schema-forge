# Repository Structure — SchemaForge

| | |
|---|---|
| **Status** | Living reference |
| **Last updated** | 2026-05-30 |
| **Related** | [system-design.md](./system-design.md) · [execution-plan.md](./execution-plan.md) · [tracker.md](./tracker.md) |

The **target** layout, built into step by step. Phase tags `(Pn)` map to
[execution-plan.md](./execution-plan.md). Each backend module starts as a single
file and is promoted to a folder (`schema.rs` → `schema/mod.rs` + submodules)
when it outgrows itself — **don't create empty files ahead of the phase that
needs them**. `(exists)` marks what the scaffold already ships.

Build order (inside-out): **schema → persistence → crud → permissions →
discovery → frontend → realtime → workflow → hardening**.

---

## Repo root

```
schema-forge/
├── README.md
├── docker-compose.yml              # Postgres + Kafka (exists)
├── .github/workflows/ci.yml        # (proposed) fmt + clippy + test / pnpm build
├── docs/                           # source of truth (exists)
├── backend/
└── frontend/
```

## Backend (Rust / Axum)

```
backend/
├── Cargo.toml · rust-toolchain.toml · .env.example   # exist
├── migrations/                     # SQLx migrations
│   ├── 0001_init.sql               # metadata tables (exists)
│   └── 000N_*.sql                  # per-entity DDL is dynamic, not stored here
├── fixtures/customer-order.json    # reference schema (exists)
├── tests/                          # integration / contract / property
│   ├── meta_schema.rs              # (exists)
│   ├── crud_api.rs                 # (P3) full CRUD cycle
│   ├── permissions.rs              # (P4) deny-by-default, masks
│   ├── discovery.rs                # (P5) contract test vs /api/schema
│   └── golden_new_entity.rs        # (P9) "<5 min new entity" flow
└── src/
    ├── main.rs · lib.rs · config.rs · error.rs · state.rs   # exist
    ├── routes.rs                   # → routes/ when it grows
    ├── schema/                     # Registry & Engine  (from schema.rs)
    │   ├── mod.rs
    │   ├── model.rs                # meta-schema types
    │   ├── engine.rs               # in-memory resolver + hot reload
    │   ├── registry.rs             # submit / activate / persistence
    │   ├── validate.rs             # (P1) full meta-schema validation
    │   └── diff.rs                 # (P1) additive vs breaking
    ├── persistence/                # (from persistence.rs)
    │   ├── mod.rs
    │   ├── adapter.rs              # hybrid columns + JSONB
    │   ├── ddl.rs                  # (P2) online DDL generation
    │   ├── query.rs                # (P3) list query builder
    │   └── migrate.rs              # (P2) breaking-change migrations
    ├── crud/                       # (from crud.rs)
    │   ├── mod.rs · handlers.rs
    │   └── validate.rs             # (P3) payload validation from constraints
    ├── permissions/                # (from permissions.rs)
    │   ├── mod.rs · evaluator.rs
    │   └── expr.rs                 # (P4) CEL sandbox wrapper
    ├── events/                     # (from events.rs)
    │   ├── mod.rs · model.rs       # DomainEvent
    │   ├── publisher.rs            # Kafka producer
    │   └── outbox.rs               # (P7) outbox relay loop
    ├── realtime/                   # (from realtime.rs)
    │   ├── mod.rs · ws.rs
    │   └── hub.rs                  # (P7) subscription registry + fanout
    ├── workflow/                   # (from workflow.rs)
    │   ├── mod.rs · engine.rs      # Kafka consumer + dispatch
    │   ├── trigger.rs
    │   └── actions.rs              # (P8) set/create/update/webhook/notify/emit
    ├── discovery/                  # (from discovery.rs)
    │   ├── mod.rs · schema_endpoint.rs
    │   └── openapi.rs              # (P5) OpenAPI 3 + JSON Schema emitters
    └── audit.rs                    # audit_log writer (cross-cutting)
```

## Frontend (React / TS / Vite)

```
frontend/
├── package.json · tsconfig.json · vite.config.ts · index.html   # exist
└── src/
    ├── main.tsx · App.tsx · index.css           # exist
    ├── api/
    │   ├── client.ts                # fetch wrapper (exists)
    │   ├── schema.ts                # fetchSchema (split out of client.ts)
    │   ├── entities.ts              # (P6) CRUD calls
    │   └── realtime.ts              # (P7) WebSocket subscription
    ├── types/schema.ts              # (exists) → later generated from OpenAPI
    ├── schema/                      # schema-driven rendering core
    │   ├── widgets.ts               # field type → widget (from components/widgets.ts)
    │   ├── validation.ts            # (P6) client validation mirror
    │   └── permissions.ts           # (P4/P6) capability + mask helpers
    ├── components/
    │   ├── SchemaForm.tsx · SchemaTable.tsx     # exist
    │   ├── EntityNav.tsx            # (split out of App.tsx)
    │   ├── RelationPicker.tsx       # (P6) relation fields
    │   └── fields/                  # (P6) per-widget field components
    │       ├── TextField.tsx · EnumField.tsx
    │       └── BooleanField.tsx · DateTimeField.tsx
    ├── hooks/
    │   ├── useSchema.ts             # load + cache /api/schema
    │   └── useEntityList.ts         # (P6) list + pagination state
    └── pages/                       # if/when routing is added
        ├── EntityListPage.tsx
        └── EntityEditPage.tsx
```
