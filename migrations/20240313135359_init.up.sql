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

CREATE TABLE profile
(
    id              uuid DEFAULT uuid_generate_v7() NOT NULL PRIMARY KEY,
    username        text                            NOT NULL,
    server          text                            NOT NULL,
    server_id       text                            NOT NULL UNIQUE,
    "display_name"  text                            NOT NULL,
    summary         text DEFAULT ''                 NOT NULL,
    inbox           text                            NOT NULL,
    outbox          text                            NOT NULL,
    "public_key"    text                            NOT NULL,
    "registered_at" timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    "updated_at"    timestamptz DEFAULT CURRENT_TIMESTAMP  NOT NULL
);

CREATE TABLE account
(
    id              uuid DEFAULT uuid_generate_v7()                                   NOT NULL PRIMARY KEY,
    "registered_at" timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
    "updated_at"    timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
    password        text                                                              NOT NULL,
    email           text                                                              NOT NULL UNIQUE,
    verified        text DEFAULT uuid_generate_v7(),
    "profile_id"    uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "private_key"   text                                                              NOT NULL
);

CREATE TABLE model
(
    id           uuid    DEFAULT uuid_generate_v7()                                NOT NULL PRIMARY KEY,
    server       text                                                              NOT NULL,
    "server_id"  text UNIQUE,
    "profile_id" uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    published    boolean DEFAULT false                                             NOT NULL,
    title        text                                                              NOT NULL,
    summary      text                                                              NOT NULL,
    description  text                                                              NOT NULL,
    tags         text[]                                                            NOT NULL DEFAULT '{}',
    "created_at" timestamptz    DEFAULT CURRENT_TIMESTAMP                                 NOT NULL,
    "updated_at" timestamptz    DEFAULT CURRENT_TIMESTAMP                                 NOT NULL
);



CREATE TABLE file
(
    id                   uuid DEFAULT uuid_generate_v7()                                   NOT NULL PRIMARY KEY,
    "created_at"         timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
    "updated_at"         timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
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
    id                       uuid DEFAULT uuid_generate_v7()                                   NOT NULL PRIMARY KEY,
    "created_at"             timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
    "updated_at"             timestamptz DEFAULT CURRENT_TIMESTAMP                                    NOT NULL,
    "server_id"              text UNIQUE,
    content                  text                                                              NOT NULL,
    hashtags                 text[]                                                            NOT NULL DEFAULT '{}',
    audience                 event_audience                                                    NOT NULL,
    "in_reply_to_comment_id" uuid                                                              REFERENCES note (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "in_reply_to_note_id"    uuid                                                              REFERENCES note (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "actor_id"               uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "comment_of_model_id"    uuid                                                              REFERENCES model (id) ON UPDATE CASCADE ON DELETE SET NULL
);



CREATE TABLE printer
(
    id                     uuid           DEFAULT uuid_generate_v7()                         NOT NULL PRIMARY KEY,
    "created_at"           timestamptz           DEFAULT CURRENT_TIMESTAMP                          NOT NULL,
    "updated_at"           timestamptz           DEFAULT CURRENT_TIMESTAMP                          NOT NULL,
    name                   text                                                              NOT NULL,
    manufacturer           text                                                              NOT NULL,
    "profile_id"           uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    public                 boolean        DEFAULT true                                       NOT NULL,
    "slicer_config"        text,
    "slicer_config_public" boolean        DEFAULT true                                       NOT NULL,
    description            text,
    "modified_scale"       modified_scale DEFAULT 'NO_MODS'::modified_scale                   NOT NULL
);


-- MANY TO MANY
CREATE TABLE _likes
(
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE,
    CONSTRAINT likes_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE _mentions
(
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE,
    CONSTRAINT mentions_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE _boosts
(
    profile_id uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES note (id) ON UPDATE CASCADE,
    CONSTRAINT boosts_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE _followers
(
    profile_id  uuid REFERENCES profile (id) ON UPDATE CASCADE ON DELETE CASCADE,
    follower_id uuid REFERENCES profile (id) ON UPDATE CASCADE,
    CONSTRAINT followers_pkey PRIMARY KEY (profile_id, follower_id)
)
