# SchemaForge — Documentation

SchemaForge is a **metadata-driven application platform**. You define entities, fields, relationships, permissions, and workflows as *schema definitions*; the platform generates the runtime — APIs, forms, tables, validation, permissions, and workflow execution — with **no entity-specific code**.

> One engine. Any number of entities. Change the schema, the app changes — no redeploy.

## Document index

| Doc | Purpose | Audience |
|-----|---------|----------|
| [PRD.md](./PRD.md) | What we're building and why; goals, users, requirements, success metrics | Product, Eng, Stakeholders |
| [system-design.md](./system-design.md) | Architecture, components, data model, APIs, technology choices | Engineering |
| [design-plan.md](./design-plan.md) | Schema language spec, key flows, edge cases, sequencing of design decisions | Engineering |
| [structure.md](./structure.md) | Target repo/folder layout (backend + frontend), phase-tagged | Engineering |
| [execution-plan.md](./execution-plan.md) | Phased delivery plan, milestones, workstreams, estimates, risks | Eng leads, PM |
| [tracker.md](./tracker.md) | Live status of epics, tasks, owners, and blockers | Everyone |

## Core capabilities

1. **Schema management** — versioned, validated definitions of entities/fields/relations.
2. **Dynamic CRUD** — generic API + persistence for any defined entity.
3. **Generated UI** — forms and tables rendered from schema, no per-entity components.
4. **Permissions** — role/attribute-based access derived from schema metadata.
5. **Workflow automation** — declarative triggers, conditions, and actions on entity events.
6. **Real-time updates** — subscriptions that push entity changes to clients.
7. **AI-ready discovery** — machine-readable schema endpoint for agents and tooling.

## Status

Greenfield. These documents are the source of truth until code exists. See the [tracker](./tracker.md) for current state.
