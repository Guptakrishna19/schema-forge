# Tracker — SchemaForge

| | |
|---|---|
| **Last updated** | 2026-05-30 |
| **Overall status** | 🟡 Phase 0 — repo scaffolded (backend + frontend), components stubbed |
| **Related** | [execution-plan.md](./execution-plan.md) · [PRD.md](./PRD.md) · [system-design.md](./system-design.md) |

Legend: ⬜ Not started · 🟡 In progress · ✅ Done · 🚧 Blocked

---

## Milestone status

| Milestone | Phase | Status | Notes |
|---|---|---|---|
| M1 — Schema lives | P1 | ⬜ | Registry + hot-load |
| M2 — Data lives | P3 | ⬜ | CRUD on any entity |
| M3 — Secure & self-describing | P5 | ⬜ | Permissions + discovery |
| M4 — Usable | P6 | ⬜ | Generated UI |
| M5 — Reactive | P8 | ⬜ | Realtime + workflows |
| M6 — v1 RC | P9 | ⬜ | Hardened, metrics met |

## Blocking decisions

| # | Decision | Status | Owner | Resolution |
|---|---|---|---|---|
| D0 | Storage strategy | ⬜ Open | TBD | Proposed: **Hybrid (columns + JSONB)** |
| D1 | Meta-schema (schema-of-schemas) | ⬜ Open | TBD | Draft in design-plan §1 |
| D2 | API style + list grammar | ⬜ Open | TBD | Proposed: **REST-first** |
| D3 | Permission grammar + expr sandbox | ⬜ Open | TBD | Grammar drafted; lib TBD |
| D4 | Event/outbox model | 🟡 Decided | TBD | **Apache Kafka** (transactional outbox → Kafka); CDC-vs-polling relay TBD |
| D5 | Workflow action set + guards | ⬜ Open | TBD | Action list drafted |
| D6 | Multitenancy model | ⬜ Open | TBD | tenant_id vs schema-per-tenant |

## Epics / phases

### Phase 0 — Foundations & decisions 🟡
- [x] Scaffold repo as **skeleton stubs** — full module/file structure, type defs, and signatures in place; every logic body is `todo!()` (Rust) / `TODO` (TS), to be filled phase by phase
- [ ] Lock D0 storage strategy
- [~] Define meta-schema (D1) — types defined in `schema.rs`; `schema::validate`/`diff` are stubs (P1)
- [x] Author `Customer`/`Order` fixture schemas (`backend/fixtures/customer-order.json`)
- [ ] CI pipeline (lint/test) — not yet
- [ ] Frontend skeleton typechecks + builds green (`pnpm build` ✅); backend build unverified (no Rust toolchain on this machine — stubs written to compile)

### Phase 1 — Schema Registry & Engine ⬜
- [ ] `schema_version` table + submit/validate/activate endpoints
- [ ] Diff engine (additive vs breaking)
- [ ] In-memory Schema Engine + hot reload

### Phase 2 — Persistence Adapter & Migrations ⬜
- [ ] Hybrid storage mapping + index/constraint generation
- [ ] Online additive DDL; relations (FK + join tables)
- [ ] Breaking-change migration step + backfill hooks

### Phase 3 — Dynamic CRUD & Query Engine ⬜
- [ ] Generic CRUD handlers
- [ ] List grammar (filter/sort/paginate/expand/fields)
- [ ] Schema validation + audit log + event emission

### Phase 4 — Permissions ⬜
- [ ] Permission Evaluator (role + expr sandbox)
- [ ] Row eligibility + field masks in CRUD
- [ ] Deny-by-default / fail-closed tests

### Phase 5 — Discovery / OpenAPI ⬜
- [ ] `/api/schema` full output
- [ ] OpenAPI 3 + JSON Schema emitters
- [ ] Agent-driven CRUD cycle test

### Phase 6 — Generated UI ⬜
- [ ] Schema-driven form renderer
- [ ] Schema-driven table renderer
- [ ] Auto navigation/admin from `/api/schema`

### Phase 7 — Real-time ⬜
- [ ] Event bus + outbox
- [ ] WebSocket/SSE hub with permission filtering
- [ ] UI live-update integration

### Phase 8 — Workflow Engine ⬜
- [ ] Trigger/condition/action model
- [ ] Outbox-backed execution (idempotent, retryable)
- [ ] Action set + loop guards + run history

### Phase 9 — Hardening & v1 ⬜
- [ ] Observability (logs/metrics/traces)
- [ ] Perf passes + index audit
- [ ] Security review + multitenancy (D6)
- [ ] Property/contract/golden-flow suites
- [ ] "< 5 min new entity" golden test

## Risks (live)

| Risk | Severity | Status | Mitigation owner |
|---|---|---|---|
| Storage strategy rework | High | Open | TBD — spike before P2 |
| JSONB query perf | Med | Open | TBD — P9 index audit |
| Expr sandbox safety | High | Open | TBD — P9 security review |
| Workflow runaway loops | Med | Open | TBD — guards in P8 |
| Migration data loss | High | Open | TBD — gated breaking changes |

## Changelog
- **2026-05-30** — Reduced the scaffold to **skeleton stubs**: kept structure, types, and signatures; replaced all logic bodies with `todo!()` / `TODO(Pn)` so the codebase is implemented step by step. Frontend still typechecks + builds.
- **2026-05-30** — Scaffolded the monorepo: `backend/` (Rust/Axum — schema engine, registry, CRUD, permissions, events/Kafka, realtime, discovery modules; SQLx migrations; meta-schema tests), `frontend/` (React/TS/Vite — generic SchemaForm/SchemaTable driven by `/api/schema`), and root `docker-compose.yml` (Postgres + Kafka). Frontend `pnpm build` green. Unbuilt-phase handlers return `501` with `TODO(Pn)` markers.
- **2026-05-30** — Stack decided: **Rust/Axum** backend, **Apache Kafka** event backbone (outbox → Kafka), **React/TS** frontend. Updated system-design §8/§3.6–3.7/§10, execution-plan P0/P7, tracker D4. SQLx for query/migrations; CEL for the expression sandbox.
- **2026-05-30** — Initial documentation set created (PRD, system design, design plan, execution plan, tracker). No code yet.
