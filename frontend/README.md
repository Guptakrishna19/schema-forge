# SchemaForge — Frontend (React / TypeScript / Vite)

A **schema-driven** UI. There are no `CustomerForm` / `OrderTable` files — only
generic `<SchemaForm>` and `<SchemaTable>` that render any entity from the
schema served at `/api/schema`. New entities appear in navigation automatically
once activated (design-plan §4).

## Stack

- React 18 + TypeScript, built with Vite
- Talks to the Rust backend via same-origin `/api/*` (proxied in dev)

## Structure

| Path | Purpose |
|---|---|
| `src/api/client.ts` | Fetch wrapper + `fetchSchema` / `listEntity` |
| `src/types/schema.ts` | TS mirror of the meta-schema (later: generated from OpenAPI) |
| `src/components/SchemaForm.tsx` | Generic form; widget per field type |
| `src/components/SchemaTable.tsx` | Generic table; columns from schema |
| `src/components/widgets.ts` | Field type → widget mapping |
| `src/App.tsx` | Discovers entities, builds nav, renders table + form |

## Run

```bash
pnpm install
pnpm dev          # http://localhost:5173, proxies /api → :8080
```

Start the backend (`cd ../backend && cargo run`) and activate the reference
schema (see `../backend/README.md`) to see entities populate the nav.

## Build / lint

```bash
pnpm build        # tsc -b && vite build
pnpm lint         # tsc --noEmit
```
