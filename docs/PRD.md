# Product Requirements Document — SchemaForge

| | |
|---|---|
| **Status** | Draft v1 |
| **Last updated** | 2026-05-30 |
| **Owner** | (TBD) |
| **Related** | [system-design.md](./system-design.md) · [execution-plan.md](./execution-plan.md) · [tracker.md](./tracker.md) |

---

## 1. Summary

SchemaForge is a metadata-driven application platform that generates a complete runtime — APIs, forms, tables, validation, permissions, and workflows — directly from **schema definitions**, with no entity-specific code. Teams describe *what* their data and rules are; SchemaForge provides *how* they are stored, served, displayed, secured, and automated.

The platform turns application development from a coding exercise into a configuration exercise, collapsing the typical "add a new entity" cycle (migration + model + API + form + table + permissions + tests + deploy) from days into minutes.

## 2. Problem statement

Most internal tools and line-of-business apps are 80% boilerplate: the same CRUD endpoints, the same form/table UI, the same permission checks, re-implemented per entity. Every new field means touching a migration, a model, a DTO, a form, a table column, and validation in several places. This:

- Slows iteration and makes business teams dependent on engineering for trivial changes.
- Produces drift between layers (DB vs API vs UI vs docs).
- Makes the system hostile to AI agents, which must reverse-engineer ad-hoc code to act on data.

## 3. Goals & non-goals

### Goals
- **G1** — Define entities, fields, relationships, permissions, and workflows purely as data.
- **G2** — Serve full CRUD + query APIs for any defined entity with zero per-entity code.
- **G3** — Render forms and data tables automatically from schema.
- **G4** — Enforce role/attribute-based permissions declared in schema.
- **G5** — Run declarative workflows on entity lifecycle events.
- **G6** — Push real-time updates to subscribed clients.
- **G7** — Expose a machine-readable schema for AI agents and external tooling.
- **G8** — Evolve schemas safely (versioning, validation, migration) without downtime.

### Non-goals (v1)
- Not a replacement for highly bespoke, performance-critical services.
- No visual drag-and-drop schema builder (API/file-driven first; UI builder is later).
- No multi-region active-active replication.
- No arbitrary user-supplied code execution inside workflows (declarative + safe expressions only).

## 4. Target users & personas

| Persona | Need | How SchemaForge helps |
|---|---|---|
| **Platform/app developer** | Ship internal apps fast | Defines schema; gets API + UI + permissions for free |
| **Business/ops analyst** | Change fields, statuses, rules | Edits schema (later via UI) without an eng ticket |
| **Integrator / AI agent** | Read/act on data programmatically | Self-describing schema + uniform API |
| **Admin / security** | Control who sees/does what | Declarative RBAC/ABAC enforced centrally |

## 5. User stories

- As a developer, I define a `Customer` entity with fields and a relation to `Order`, and immediately get `/api/customers` CRUD plus a working form and table.
- As an analyst, I add a `priority` field to `Ticket` and it appears in the form, table, and API without a deploy.
- As an admin, I declare that `Agents` can read but not delete `Invoice`, and the rule is enforced everywhere.
- As an ops user, I declare "when an Order's status becomes `paid`, create a `Shipment` and notify the warehouse channel."
- As a dashboard, I subscribe to `Order` changes and see new rows appear live.
- As an AI agent, I fetch `/api/schema` and know every entity, field type, relation, and allowed action without human docs.

## 6. Functional requirements

### 6.1 Schema management
- FR-1 Define entities with named, typed fields (string, number, boolean, datetime, enum, json, reference, etc.).
- FR-2 Per-field constraints: required, unique, default, min/max, pattern, enum values.
- FR-3 Relationships: one-to-one, one-to-many, many-to-many, with referential rules.
- FR-4 Schemas are **versioned**; changes are validated before activation.
- FR-5 Backward-incompatible changes are detected and require an explicit migration step.

### 6.2 Dynamic CRUD & query
- FR-6 Generic REST (and/or GraphQL) endpoints for every entity: create, read, update, delete, list.
- FR-7 List supports filtering, sorting, pagination, field selection, and relation expansion.
- FR-8 Server-side validation derived from schema constraints.

### 6.3 Generated UI
- FR-9 Auto-generated forms with widgets chosen by field type and constraints.
- FR-10 Auto-generated tables with sort/filter/paginate, column visibility from schema.
- FR-11 Relation fields render as pickers; client validation mirrors server rules.

### 6.4 Permissions
- FR-12 Roles and rules declared in schema metadata (per entity, per action, per field).
- FR-13 Attribute-based rules (e.g., "owner can edit") via safe expressions.
- FR-14 Enforced uniformly at the API layer; UI hides/disables disallowed actions.

### 6.5 Workflow automation
- FR-15 Triggers on entity lifecycle events (create/update/delete, field change, schedule).
- FR-16 Declarative conditions and ordered actions (set field, create/update entity, call webhook, send notification).
- FR-17 Execution is observable (run history, status, errors) and ret-safe.

### 6.6 Real-time
- FR-18 Clients subscribe to entity/collection changes and receive push updates.
- FR-19 Subscriptions respect the same permission rules as the API.

### 6.7 AI-ready discovery
- FR-20 A single endpoint returns the full, machine-readable schema (entities, fields, relations, actions, permissions metadata).
- FR-21 Output is consumable as JSON Schema / OpenAPI so agents and tools can self-configure.

## 7. Non-functional requirements
- **Performance**: dynamic CRUD p95 < 150 ms for simple reads at moderate load; generated queries must be index-aware.
- **Reliability**: schema changes must never corrupt existing data; workflows are idempotent/retryable.
- **Security**: deny-by-default permissions; safe expression sandbox (no arbitrary code); audit log of schema and data changes.
- **Scalability**: thousands of entities and tens of millions of records per tenant.
- **Extensibility**: new field types, widgets, and workflow actions added via plugins without core changes.
- **Observability**: structured logs, metrics, and traces for API, workflow, and real-time layers.

## 8. Assumptions & dependencies
- A relational store with strong JSON support (PostgreSQL + JSONB) is acceptable as the primary store.
- Single-tenant-per-deployment or shared multi-tenant with tenant scoping (see system-design).
- Clients can speak WebSocket/SSE for real-time.

## 9. Success metrics
- **Time-to-new-entity**: define + serve a usable entity (API + form + table) in **< 5 minutes**.
- **Zero per-entity code**: 0 lines of entity-specific backend/frontend code for standard cases.
- **Adoption**: N internal apps migrated onto SchemaForge in first 2 quarters.
- **Change velocity**: median field/rule change shipped without an engineering deploy.
- **Agent readiness**: an external agent can perform a full CRUD cycle using only `/api/schema`.

## 10. Open questions
- REST-first, GraphQL-first, or both? (Leaning REST + schema discovery; GraphQL as fast-follow.)
- Multi-tenancy model: shared schema vs schema-per-tenant?
- How far does the safe-expression language go before it needs sandboxed code?
- Migration UX for breaking changes — automatic vs reviewed?

## 11. Out of scope for v1 (future)
- Visual schema builder UI, computed/derived fields, full-text & vector search, marketplace of schema templates, offline-first clients.
