// TypeScript mirror of the backend meta-schema (docs/design-plan.md §1).
// In a later phase these types are generated from the backend's OpenAPI/JSON
// Schema output (system-design §8) instead of being hand-maintained.

export type FieldType =
  | "string"
  | "text"
  | "integer"
  | "decimal"
  | "boolean"
  | "datetime"
  | "date"
  | "enum"
  | "json"
  | "reference";

export interface FieldDef {
  type: FieldType;
  required?: boolean;
  unique?: boolean;
  default?: unknown;
  min?: number;
  max?: number;
  minLength?: number;
  maxLength?: number;
  pattern?: string;
  format?: string;
  values?: string[];
}

export type RelationKind =
  | "one-to-one"
  | "one-to-many"
  | "many-to-one"
  | "many-to-many";

export interface RelationDef {
  kind: RelationKind;
  target: string;
  inverse?: string;
  on_delete?: "restrict" | "cascade" | "set-null";
}

export interface IndexDef {
  fields: string[];
  unique?: boolean;
}

export interface EntityDef {
  fields: Record<string, FieldDef>;
  relations?: Record<string, RelationDef>;
  permissions?: Record<string, string[]>;
  indexes?: IndexDef[];
}

export interface WorkflowDef {
  name: string;
  on: { entity: string; event: string; when?: string };
  actions?: unknown[];
}

export interface Schema {
  version: number | null;
  entities: Record<string, EntityDef>;
  workflows?: WorkflowDef[];
  note?: string;
}
