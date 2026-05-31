-- SchemaForge metadata tables (system-design §4).
-- Per-entity data tables are created dynamically by the Persistence Adapter (P2).

-- Versioned schema definitions; immutable once activated.
CREATE TABLE IF NOT EXISTS schema_version (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    version      INTEGER NOT NULL,
    status       TEXT NOT NULL CHECK (status IN ('draft', 'active', 'archived')),
    definition   JSONB NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    activated_at TIMESTAMPTZ
);

-- Only one active version at a time.
CREATE UNIQUE INDEX IF NOT EXISTS schema_version_one_active
    ON schema_version (status) WHERE status = 'active';

-- Audit log of schema and data changes.
CREATE TABLE IF NOT EXISTS audit_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor       TEXT,
    action      TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id   TEXT,
    before      JSONB,
    after       JSONB,
    at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Workflow execution history.
CREATE TABLE IF NOT EXISTS workflow_run (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id TEXT NOT NULL,
    trigger     TEXT NOT NULL,
    entity      TEXT,
    record_id   TEXT,
    status      TEXT NOT NULL,
    steps       JSONB,
    error       TEXT,
    at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Transactional outbox: written in the same tx as the data change, then
-- relayed to Kafka by the outbox relay (system-design §3.7).
CREATE TABLE IF NOT EXISTS outbox (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type         TEXT NOT NULL,
    payload      JSONB NOT NULL,
    status       TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed')),
    attempts     INTEGER NOT NULL DEFAULT 0,
    available_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS outbox_pending ON outbox (available_at) WHERE status = 'pending';
