# Design Plan — SchemaForge

| | |
|---|---|
| **Status** | Draft v1 |
| **Last updated** | 2026-05-30 |
| **Related** | [system-design.md](./system-design.md) · [PRD.md](./PRD.md) |

This document specifies **the schema definition language** (the heart of the product), the **key end-to-end flows**, the **important design tradeoffs**, and the **order in which design decisions must be locked** so workstreams can proceed without rework.

---

## 1. The schema definition language

A schema is a JSON/YAML document describing entities. This is the contract every subsystem reads.

### 1.1 Example schema

```yaml
version: 3
entities:
  Customer:
    fields:
      name:    { type: string, required: true, maxLength: 200 }
      email:   { type: string, required: true, unique: true, format: email }
      tier:    { type: enum, values: [free, pro, enterprise], default: free }
      notes:   { type: text }
    relations:
      orders:  { kind: one-to-many, target: Order, inverse: customer }
    permissions:
      read:    [ "role:staff", "expr:record.ownerId == user.id" ]
      create:  [ "role:staff" ]
      update:  [ "role:staff" ]
      delete:  [ "role:admin" ]
    indexes:
      - { fields: [email], unique: true }

  Order:
    fields:
      total:   { type: decimal, required: true, min: 0 }
      status:  { type: enum, values: [draft, paid, shipped, cancelled], default: draft }
      paidAt:  { type: datetime }
    relations:
      customer: { kind: many-to-one, target: Customer }
    permissions:
      read:    [ "role:staff" ]
      "*":     [ "role:staff" ]

workflows:
  - name: ship-on-paid
    on: { entity: Order, event: update, when: "record.status == 'paid' && prior.status != 'paid'" }
    actions:
      - set:    { field: paidAt, value: "now()" }
      - create: { entity: Shipment, with: { orderId: "record.id" } }
      - notify: { channel: "warehouse", template: "order_paid" }
```

### 1.2 Field types (v1)
`string`, `text`, `integer`, `decimal`, `boolean`, `datetime`, `date`, `enum`, `json`, `reference` (relation). Each type maps to: a storage representation, a validator, a form widget, and a table renderer.

### 1.3 Field constraints
`required`, `unique`, `default`, `min`/`max`, `minLength`/`maxLength`, `pattern`, `format`, `values` (enum). Constraints generate **both** server validation and client validation.

### 1.4 Relationships
- `many-to-one` / `one-to-many` (FK column on the "many" side).
- `one-to-one` (FK + unique).
- `many-to-many` (generated join table).
- `inverse` names the back-reference; referential rules: `restrict | cascade | set-null`.

### 1.5 Permissions grammar
Each (entity, action) maps to a list of **grants**; access is allowed if **any** grant matches.
- `role:<name>` — subject has the role.
- `expr:<expression>` — sandboxed boolean over `user`, `record`, `prior`, `now()`.
- Field-level overrides: `permissions.fields.<field>.{read,write}` produce read/write masks.
- Absence ⇒ deny.

### 1.6 Workflow grammar
- `on`: `{ entity, event: create|update|delete|schedule, when: <expr> }`.
- `actions`: ordered list of `set | create | update | delete | webhook | notify | emit`.
- Expressions reference `record`, `prior`, `user`, `now()`. No loops, no arbitrary code.

## 2. Storage strategy decision (must lock first)

| Option | Pros | Cons |
|---|---|---|
| **A. Pure JSONB** (`data JSONB` only) | Schema changes need no DDL; fastest to build | Weaker constraints, indexing harder, type fidelity in queries |
| **B. Hybrid** (known fields → columns, rest → JSONB) ✅ default | Indexable/queryable core, flexible extension, good constraints | Migration needed for column promotion; more adapter logic |
| **C. Per-entity physical tables** | Best query perf & integrity | Heavy DDL on every change; closest to what we're trying to avoid |

**Recommendation: B (Hybrid).** Promote a field to a real column when it is unique/indexed/filtered-often; keep the rest in JSONB. The Persistence Adapter hides this from all other layers. This decision drives the migration engine and query builder, so it is **Decision 0**.

## 3. Key flows

### 3.1 "Add a new entity" (the headline flow)
1. Author adds an entity to the schema doc → submits as a candidate version.
2. Registry validates + diffs (additive) → activates.
3. Adapter creates table/columns/indexes online.
4. Schema Engine hot-reloads → CRUD routes, OpenAPI, form, and table are live.
5. UI auto-discovers the new entity from `/api/schema` and lists it in navigation.

### 3.2 Add a field to an existing entity
Additive change → online `ADD COLUMN` (or JSONB key, no DDL) → appears in form/table/API. No deploy.

### 3.3 Breaking change (drop field / narrow type / add required)
Blocked from auto-activation → requires migration plan (backfill/transform) → run in controlled window → activate.

### 3.4 Permission-filtered read
CRUD read → Permission Evaluator computes row eligibility + field read-mask → masked record returned. Realtime hub applies the **same** mask before pushing.

### 3.5 Workflow on event
Commit emits event → Workflow Engine matches trigger → evaluates `when` → runs actions via outbox (idempotent, retryable) → records `workflow_run`.

## 4. UI rendering plan
- **Form renderer**: maps field type+constraints → widget; renders relation pickers; mirrors server validation; respects write-mask (disable/hide).
- **Table renderer**: columns from schema, sort/filter/paginate via list API; respects read-mask.
- **Navigation/admin**: built from `/api/schema`; new entities appear automatically.
- Components are **generic** — there are no `CustomerForm`/`OrderTable` files; only `<SchemaForm entity>` / `<SchemaTable entity>`.

## 5. Edge cases & how they're handled
- **Unknown entity/field in request** → 404/422 from Schema Engine.
- **Concurrent schema activation** → registry serializes activations; engine reload is atomic.
- **Relation to not-yet-defined entity** → validation error at submit.
- **Workflow infinite loops** (A updates B updates A) → depth/iteration guard + dedupe by run key.
- **Permission expression error** → treated as deny + logged (fail-closed).
- **Large list expansion (N+1)** → adapter batches relation loads; cap expand depth.
- **Schema rollback** → previous active version retained; re-activate; data-compatibility checked.

## 6. Decision sequencing (lock order)
These gate downstream work; lock in this order:

| # | Decision | Blocks |
|---|---|---|
| D0 | Storage strategy (→ Hybrid) | Adapter, migration engine, query builder |
| D1 | Schema language schema (the meta-schema) | Registry, engine, UI, discovery |
| D2 | API style (REST-first) + list query grammar | CRUD engine, UI, OpenAPI |
| D3 | Permission grammar + expression sandbox | Evaluator, CRUD, realtime, UI masks |
| D4 | Event/outbox model | Workflow engine, realtime hub |
| D5 | Workflow action set + safety guards | Workflow engine |
| D6 | Multitenancy model | All persistence + permission layers |

## 7. Validation & quality strategy
- **Meta-schema validation** of every schema doc (the schema-of-schemas).
- **Property-based tests**: generate random valid schemas, assert CRUD + validation + permissions hold.
- **Contract tests** against `/api/schema` and OpenAPI output.
- **Golden flows**: the "add entity in < 5 min" flow is an automated end-to-end test.
