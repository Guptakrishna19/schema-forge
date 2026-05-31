// Generic, schema-driven table. Columns derive from the entity's fields; rows
// come from the list API. No per-entity table component (design-plan §4).

import type { EntityDef } from "../types/schema";

interface Props {
  entity: string;
  def: EntityDef;
}

export function SchemaTable({ entity, def }: Props) {
  // TODO(P6): columns from def.fields, rows from listEntity(entity),
  // sort/filter/paginate, read-mask, loading/error/empty states.
  return null;
}
