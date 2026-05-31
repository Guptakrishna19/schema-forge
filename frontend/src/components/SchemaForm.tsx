// Generic, schema-driven form. There is no per-entity form component — this
// renders any entity from its schema definition (design-plan §4).

import type { EntityDef } from "../types/schema";

interface Props {
  entity: string;
  def: EntityDef;
}

export function SchemaForm({ entity, def }: Props) {
  // TODO(P6): render an input per field (widgetFor), client validation from
  // constraints, relation pickers, write-mask, submit → POST /api/{entity}.
  return null;
}
