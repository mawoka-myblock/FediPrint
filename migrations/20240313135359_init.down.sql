-- This file should undo anything in `up.sql`
DROP FUNCTION IF EXISTS manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS set_updated_at();
DROP TABLE likes;
DROP TABLE boosts;
DROP TABLE _mentions;
DROP TABLE followers;
DROP TABLE printer;
DROP TABLE note;
DROP TABLE file;
DROP TABLE model;
DROP TABLE account;
DROP TABLE profile;
DROP TYPE modified_scale;
DROP TYPE event_audience;
DROP TYPE model_license;
DROP EXTENSION pg_uuidv7;
