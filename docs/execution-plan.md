# Execution Plan — SchemaForge

| | |
|---|---|
| **Status** | Draft v1 |
| **Last updated** | 2026-05-30 |
| **Related** | [PRD.md](./PRD.md) · [system-design.md](./system-design.md) · [tracker.md](./tracker.md) |

Phased delivery from greenfield to a self-describing, workflow-capable platform. Each phase ends in a **demoable, testable increment**. Estimates assume a small team (2–3 engineers) and are relative, not committed dates.

---

## Guiding sequence

Build inside-out: **schema → storage → CRUD → permissions → UI → realtime → workflows → discovery**. Lock the decisions in [design-plan.md §6](./design-plan.md#6-decision-sequencing-lock-order) before the phase that depends on them.

## Phase 0 — Foundations & decisions *(~1 week)*
**Goal:** repo, CI, and all blocking design decisions locked.
- Scaffold repo (Rust/Axum backend, React/TS frontend, Postgres, Kafka, test harness, lint, CI).
- Lock D0 storage strategy (Hybrid) and D1 meta-schema.
- Define the meta-schema (schema-of-schemas) + validator.
- Author 2 reference schemas (`Customer`/`Order`) as fixtures.
**Exit:** `npm test` green; meta-schema validates fixtures; decisions D0–D1 recorded in tracker.

## Phase 1 — Schema Registry & Engine *(~1.5 weeks)*
**Goal:** versioned, validated schemas that hot-load.
- `schema_version` table; submit/validate/activate endpoints.
- Diff engine (additive vs breaking classification).
- In-memory Schema Engine with hot reload on activation.
**Exit:** activate a schema version via API; engine reflects it; breaking changes rejected.

## Phase 2 — Persistence Adapter & Migrations *(~2 weeks)*
**Goal:** schema → physical storage automatically.
- Hybrid mapping (columns + JSONB), index/constraint generation.
- Online additive DDL on activation; relations (FK + join tables).
- Migration step for breaking changes (manual plan + backfill hooks).
**Exit:** activating a new entity creates its storage; adding a field migrates online.

## Phase 3 — Dynamic CRUD & Query Engine *(~2 weeks)*
**Goal:** full API for any entity, zero per-entity code.
- Generic create/read/update/delete/list handlers.
- List grammar: filter/sort/paginate/expand/fields.
- Schema-derived server validation; audit log; emit domain events.
**Exit:** full CRUD cycle on fixtures via REST; validation enforced; events emitted.

## Phase 4 — Permissions *(~1.5 weeks)*
**Goal:** declarative RBAC/ABAC enforced everywhere.
- Permission Evaluator; role + `expr:` sandbox.
- Row eligibility + field read/write masks wired into CRUD.
- Deny-by-default; fail-closed on errors.
**Exit:** rules in schema gate API actions and mask fields; negative tests pass.

## Phase 5 — Discovery / OpenAPI *(~1 week)*
**Goal:** the platform describes itself.
- `/api/schema` full machine-readable output.
- OpenAPI 3 + JSON Schema emitters.
**Exit:** an external client (and an AI agent) performs a CRUD cycle using only discovery output.

## Phase 6 — Generated UI *(~2.5 weeks)*
**Goal:** forms + tables with no per-entity components.
- Schema-driven form renderer (widgets by type/constraint, relation pickers, client validation).
- Schema-driven table (sort/filter/paginate, masks).
- Auto navigation/admin from `/api/schema`.
**Exit:** create/edit/list any entity in the UI; new entity appears automatically.

## Phase 7 — Real-time *(~1.5 weeks)*
**Goal:** live updates respecting permissions.
- Transactional outbox → Kafka relay; Kafka-consuming WebSocket/SSE hub.
- Subscriptions with per-client permission filtering.
- UI tables subscribe and update live.
**Exit:** a change in one client appears live in another, masked correctly.

## Phase 8 — Workflow Engine *(~2.5 weeks)*
**Goal:** declarative automation on events.
- Trigger/condition/action model; outbox-backed execution (idempotent, retryable).
- Action set: set/create/update/delete/webhook/notify/emit; loop guards.
- Run history + observability.
**Exit:** the `ship-on-paid` workflow runs end-to-end with retries and run history.

## Phase 9 — Hardening & v1 readiness *(~2 weeks)*
**Goal:** production-grade.
- Observability (logs/metrics/traces), perf passes (p95 targets), index audit.
- Security review (sandbox, SQL safety, authz), multitenancy decision (D6) implemented.
- Property/contract/golden-flow test suites; docs; the "< 5 min new entity" golden test green.
**Exit:** v1 release candidate meets PRD success metrics & NFRs.

---

## Workstreams (parallelizable)
- **Backend core** — registry, adapter, CRUD, permissions, discovery (Phases 1–5).
- **Workflow & realtime** — event bus, hub, workflow engine (Phases 7–8), can start design during Phase 3.
- **Frontend** — schema renderer (Phase 6), can prototype against fixtures during Phase 3.
- **Platform** — CI, observability, security, perf (Phase 0 + Phase 9 continuous).

## Milestones
- **M1 — "Schema lives"** (end P1): activate/hot-load schemas.
- **M2 — "Data lives"** (end P3): full CRUD on any entity, no per-entity code.
- **M3 — "Secure & self-describing"** (end P5): permissions + discovery; agents can drive it.
- **M4 — "Usable"** (end P6): generated UI.
- **M5 — "Reactive"** (end P8): realtime + workflows.
- **M6 — "v1 RC"** (end P9): hardened, metrics met.

## Risks & mitigations
| Risk | Impact | Mitigation |
|---|---|---|
| Storage strategy wrong | High rework | Lock D0 early; adapter hides strategy; spike before P2 |
| Query perf on JSONB | Latency misses | Promote hot fields to columns; index audit in P9 |
| Permission expr sandbox unsafe | Security breach | Use vetted expression lib; fail-closed; security review P9 |
| Workflow loops / runaway | Outages | Depth guards, dedupe keys, run quotas |
| Migration breaks data | Data loss | Breaking changes gated; backfill hooks; rollback via version history |
| Scope creep (GraphQL, UI builder) | Slip | Explicitly deferred in PRD §11 |

## Definition of done (per phase)
Tests written + green · feature flag/clean activation · docs + tracker updated · demo recorded · no open P0/P1 bugs.
