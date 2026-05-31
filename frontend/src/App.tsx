// Shell: discovers entities from /api/schema and renders the generic table +
// form for the selected one. Navigation is built entirely from the schema, so a
// new entity appears automatically once activated (design-plan §3.1, §4).

import "./index.css";

export default function App() {
  // TODO(P6): fetchSchema() → build EntityNav from entity names → render
  // <SchemaTable> + <SchemaForm> for the selected entity; loading/error states.
  return (
    <div className="sf-app">
      <header className="sf-header">
        <h1>SchemaForge</h1>
      </header>
      <p>TODO(P6): schema-driven UI shell.</p>
    </div>
  );
}
