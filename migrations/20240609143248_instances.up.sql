-- Add up migration script here


CREATE TABLE instances (
    id uuid DEFAULT uuid_generate_v7() NOT NULL PRIMARY KEY,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    base_url text NOT NULL UNIQUE,
    instance_name text,
    user_count int DEFAULT NULL,
    software text NOT NULL,
    software_version text
);
