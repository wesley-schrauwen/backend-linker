CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "links" (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    target_url TEXT NOT NULL,
    shortened_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    version INT NOT NULL
);
