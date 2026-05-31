// Maps a schema field type to the HTML input attributes for its form widget.
// Centralizes the "field type → widget" rule from design-plan §4 so both the
// form and (later) inline editors agree.

import type { FieldDef } from "../types/schema";

export interface WidgetSpec {
  /** `<input type>` (or "textarea" / "select" / "checkbox"). */
  control: "text" | "number" | "checkbox" | "datetime-local" | "date" | "textarea" | "select";
  inputType?: string;
}

export function widgetFor(field: FieldDef): WidgetSpec {
  // TODO(P6): switch on field.type → widget control.
  throw new Error("TODO(P6): map field type to widget");
}
