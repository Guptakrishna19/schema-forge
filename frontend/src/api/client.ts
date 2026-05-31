// Thin API client over the SchemaForge backend. Paths are same-origin and
// proxied to the Rust backend in dev (see vite.config.ts).

import type { Schema } from "../types/schema";

/** Fetch the full machine-readable active schema (drives the entire UI). */
export function fetchSchema(): Promise<Schema> {
  throw new Error("TODO(P5): GET /api/schema");
}

/** List records for an entity (filter/sort/paginate params land in P6). */
export function listEntity(entity: string): Promise<{ data: unknown[] }> {
  throw new Error("TODO(P6): GET /api/{entity}");
}
