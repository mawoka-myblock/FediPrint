-- Add down migration script here
DROP TABLE jobs;
DROP TYPE job_status;
DROP TYPE job_type;
DROP FUNCTION IF EXISTS notify_worker_update;
DROP FUNCTION IF EXISTS notify_past_retry;
