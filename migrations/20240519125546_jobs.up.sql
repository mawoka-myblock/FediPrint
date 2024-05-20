-- Add up migration script here

CREATE OR REPLACE FUNCTION notify_worker_update() RETURNS TRIGGER AS
$$
BEGIN
    PERFORM pg_notify('worker_update', NEW.id::TEXT);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_past_retry() RETURNS TRIGGER AS $$
BEGIN
    -- Check if the retry_at value is in the past
    IF NEW.retry_at < NOW() THEN
        PERFORM pg_notify('worker_retry', NEW.id::TEXT);
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TYPE job_status AS ENUM (
    'UNPROCESSED',
    'PROCESSING',
    'FINISHED',
    'WAITING_FOR_RETRY',
    'FAILED'
    );

CREATE TYPE job_type AS ENUM (
    'SEND_REGISTER_EMAIL'
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
    failure_log      TEXT[]      DEFAULT '{}'                      NOT NULL,
    tries            INTEGER     DEFAULT 0                         NOT NULL,
    max_tries        INTEGER     DEFAULT 3                         NOT NULL,
    processing_times FLOAT[]     DEFAULT '{}'                      NOT NULL,
    updated_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP         NOT NULL,
    job_type         job_type                                      NOT NULL
);


CREATE TRIGGER worker_update_trigger
    AFTER INSERT
    ON jobs
    FOR EACH ROW
EXECUTE FUNCTION notify_worker_update();

CREATE TRIGGER retry_at_trigger
    AFTER INSERT OR UPDATE ON jobs
    FOR EACH ROW
    WHEN (NEW.retry_at < NOW())
EXECUTE FUNCTION notify_past_retry();
