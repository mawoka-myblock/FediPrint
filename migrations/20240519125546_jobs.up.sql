-- Add up migration script here

CREATE TYPE job_status AS ENUM (
    'UNPROCESSED',
    'PROCESSING',
    'FINISHED',
    'WAITING_FOR_RETRY'
    );

CREATE TABLE jobs
(
    id               INTEGER PRIMARY KEY,
    created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP         NOT NULL,
    started_at       TIMESTAMPTZ DEFAULT NULL,
    status           JOB_STATUS  DEFAULT 'UNPROCESSED'::JOB_STATUS NOT NULL,
    retry_at         TIMESTAMPTZ DEFAULT NULL,
    finished_at      TIMESTAMPTZ DEFAULT NULL,
    input_data       TEXT        DEFAULT NULL,
    return_data      TEXT        DEFAULT NULL,
    tries            INTEGER     DEFAULT 0,
    max_tries        INTEGER     DEFAULT 3                         NOT NULL,
    processing_times FLOAT[]     DEFAULT '{}'                      NOT NULL,
    updated_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP         NOT NULL,
    job_type         INTEGER                                       NOT NULL
)
