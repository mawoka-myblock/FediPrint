CREATE OR REPLACE FUNCTION manage_updated_at(_tbl regclass) RETURNS VOID AS
$$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS
$$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
        ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;



CREATE EXTENSION pg_uuidv7;

CREATE TYPE event_audience AS ENUM (
    'PUBLIC',
    'FOLLOWERS',
    'MENTIONED',
    'NOBODY'
    );
CREATE TYPE modified_scale AS ENUM (
    'NO_MODS',
    'LIGHT_MODS',
    'MEDIUM_MODS',
    'HARD_MODS',
    'NEW_PRINTER'
    );

CREATE TYPE model_license AS ENUM (
    'CC_PD',
    'CC_ATTR',
    'CC_ATTR_SA',
    'CC_ATTR_ND',
    'CC_ATTR_NC',
    'CC_ATTR_NC_SA',
    'CC_ATTR_NC_ND',
    'GPL2',
    'GPL3',
    'GNU_LESSER',
    'BSD',
    'SDFL'
    );

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


CREATE TABLE profile
(
    id                          uuid        DEFAULT uuid_generate_v7() NOT NULL PRIMARY KEY,
    username                    text                                   NOT NULL,
    server                      text                                   NOT NULL,
    server_id                   text                                   NOT NULL UNIQUE,
    "display_name"              text                                   NOT NULL,
    summary                     text        DEFAULT ''                 NOT NULL,
    inbox                       text                                   NOT NULL,
    outbox                      text                                   NOT NULL,
    "public_key"                text                                   NOT NULL,
    "registered_at"             timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    "updated_at"                timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    "linked_printables_profile" text        DEFAULT Null UNIQUE,
    instance                    uuid REFERENCES instances(id)ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL
);

CREATE TABLE account
(
    id              uuid        DEFAULT uuid_generate_v7()                            NOT NULL PRIMARY KEY,
    "registered_at" timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "updated_at"    timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    password        text                                                              NOT NULL,
    email           text                                                              NOT NULL UNIQUE,
    verified        text        DEFAULT uuid_generate_v7(),
    "profile_id"    uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "private_key"   text                                                              NOT NULL
);

CREATE TABLE model
(
    id             uuid        DEFAULT uuid_generate_v7()                            NOT NULL PRIMARY KEY,
    server         text                                                              NOT NULL,
    "server_id"    text UNIQUE,
    "profile_id"   uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    published      boolean     DEFAULT false                                         NOT NULL,
    title          text                                                              NOT NULL,
    summary        text                                                              NOT NULL,
    description    text                                                              NOT NULL,
    tags           text[]                                                            NOT NULL DEFAULT '{}',
    license        model_license                                                     NOT NULL,
    "created_at"   timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "updated_at"   timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    printables_url text        DEFAULT NULL UNIQUE
);



CREATE TABLE file
(
    id                   uuid        DEFAULT uuid_generate_v7()                            NOT NULL PRIMARY KEY,
    "created_at"         timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "updated_at"         timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "mime_type"          text                                                              NOT NULL,
    size                 bigint                                                            NOT NULL,
    "file_name"          text,
    description          text,
    "alt_text"           text,
    thumbhash            text,
    "preview_file_id"    uuid                                                              REFERENCES file (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "to_be_deleted_at"   timestamptz DEFAULT (now() + '30 days'::interval),
    "profile_id"         uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    "file_for_model_id"  uuid                                                              REFERENCES model (id) ON UPDATE CASCADE ON DELETE SET NULL,
    "image_for_model_id" uuid                                                              REFERENCES model (id) ON UPDATE CASCADE ON DELETE SET NULL
);



CREATE TABLE note
(
    id                       uuid        DEFAULT uuid_generate_v7()                            NOT NULL PRIMARY KEY,
    "created_at"             timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "updated_at"             timestamptz DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    "server_id"              text UNIQUE,
    content                  text                                                              NOT NULL,
    hashtags                 text[]                                                            NOT NULL DEFAULT '{}',
    audience                 event_audience                                                    NOT NULL,
    "in_reply_to_comment_id" uuid                                                              REFERENCES note (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "in_reply_to_note_id"    uuid                                                              REFERENCES note (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "in_reply_to_model_id"   uuid                                                              REFERENCES model (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "actor_id"               uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "comment_of_model_id"    uuid                                                              REFERENCES model (id) ON UPDATE CASCADE ON DELETE SET NULL
);



CREATE TABLE printer
(
    id                     uuid           DEFAULT uuid_generate_v7()                         NOT NULL PRIMARY KEY,
    "created_at"           timestamptz    DEFAULT CURRENT_TIMESTAMP                          NOT NULL,
    "updated_at"           timestamptz    DEFAULT CURRENT_TIMESTAMP                          NOT NULL,
    name                   text                                                              NOT NULL,
    manufacturer           text                                                              NOT NULL,
    "profile_id"           uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    public                 boolean        DEFAULT true                                       NOT NULL,
    "slicer_config"        text,
    "slicer_config_public" boolean        DEFAULT true                                       NOT NULL,
    description            text,
    "modified_scale"       modified_scale DEFAULT 'NO_MODS'::modified_scale                  NOT NULL
);
ALTER TABLE printer
    ADD UNIQUE (name, manufacturer, profile_id);

CREATE TABLE likes
(
    id         uuid        DEFAULT uuid_generate_v7()                           NOT NULL PRIMARY KEY,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP                            NOT NULL,
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    model_id   uuid REFERENCES model (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE ON DELETE CASCADE
);





-- MANY TO MANY


CREATE TABLE _mentions
(
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT mentions_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE boosts
(
    id         uuid        DEFAULT uuid_generate_v7() NOT NULL PRIMARY KEY,
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL
);

CREATE TABLE followers
(
    id          uuid        DEFAULT uuid_generate_v7() NOT NULL PRIMARY KEY,
    profile_id  uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    follower_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at  timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL
);





-----------------------------------------
-- START JOB SECTION
-----------------------------------------


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

-----------------------------------------
-- END JOB SECTION
-----------------------------------------
