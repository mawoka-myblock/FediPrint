CREATE EXTENSION pg_uuidv7;

CREATE TYPE "EventAudience" AS ENUM (
    'Public',
    'Followers',
    'Mentioned',
    'Nobody'
    );
CREATE TYPE "ModifiedScale" AS ENUM (
    'NoMods',
    'LightMods',
    'MediumMods',
    'HardMods',
    'NewPrinter'
    );

CREATE TABLE "Profile"
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
    "registered_at" date DEFAULT CURRENT_TIMESTAMP  NOT NULL,
    "updated_at"    timestamp(3) without time zone  NOT NULL
);

CREATE TABLE "Account"
(
    id              uuid                           DEFAULT uuid_generate_v7()           NOT NULL PRIMARY KEY,
    "registered_at" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP            NOT NULL,
    "updated_at"    timestamp(3) without time zone                                      NOT NULL,
    password        text                                                                NOT NULL,
    email           text                                                                NOT NULL UNIQUE,
    verified        text                           DEFAULT uuid_generate_v7(),
    "profile_id"    uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "private_key"   text                                                                NOT NULL
);

CREATE TABLE "Model"
(
    id           uuid                           DEFAULT uuid_generate_v7()           NOT NULL PRIMARY KEY,
    server       text                                                                NOT NULL,
    "server_id"  text UNIQUE,
    "profile_id" uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    published    boolean                        DEFAULT false                        NOT NULL,
    title        text                                                                NOT NULL,
    summary      text                                                                NOT NULL,
    description  text                                                                NOT NULL,
    tags         text[]                                                              NOT NULL DEFAULT '{}',
    "created_at" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP            NOT NULL,
    "updated_at" timestamp(3) without time zone                                      NOT NULL
);



CREATE TABLE "File"
(
    id                   uuid                           DEFAULT uuid_generate_v7()           NOT NULL PRIMARY KEY,
    "created_at"         timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP            NOT NULL,
    "updated_at"         timestamp(3) without time zone                                      NOT NULL,
    "mime_type"          text                                                                NOT NULL,
    size                 bigint                                                              NOT NULL,
    "file_name"          text,
    description          text,
    "alt_text"           text,
    thumbhash            text,
    "preview_file_id"    uuid                                                                REFERENCES "File" (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "to_be_deleted_at"   timestamp(3) without time zone DEFAULT (now() + '30 days'::interval),
    "profile_id"         uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    "file_for_model_id"  uuid                                                                REFERENCES "Model" (id) ON UPDATE CASCADE ON DELETE SET NULL,
    "image_for_model_id" uuid                                                                REFERENCES "Model" (id) ON UPDATE CASCADE ON DELETE SET NULL
);



CREATE TABLE "Note"
(
    id                       uuid                           DEFAULT uuid_generate_v7()           NOT NULL PRIMARY KEY,
    "created_at"             timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP            NOT NULL,
    "updated_at"             timestamp(3) without time zone                                      NOT NULL,
    "server_id"              text UNIQUE,
    content                  text                                                                NOT NULL,
    hashtags                 text[] NOT NULL DEFAULT '{}',
    audience                 "EventAudience"                                                     NOT NULL,
    "in_reply_to_comment_id" uuid                                                                REFERENCES "Note" (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "in_reply_to_note_id"    uuid                                                                REFERENCES "Note" (id) ON UPDATE CASCADE ON DELETE SET NULL UNIQUE,
    "actor_id"               uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL UNIQUE,
    "comment_of_model_id"    uuid                                                                REFERENCES "Model" (id) ON UPDATE CASCADE ON DELETE SET NULL
);



CREATE TABLE "Printer"
(
    id                     uuid                           DEFAULT uuid_generate_v7()           NOT NULL PRIMARY KEY,
    "created_at"           timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP            NOT NULL,
    "updated_at"           timestamp(3) without time zone                                      NOT NULL,
    name                   text                                                                NOT NULL,
    manufacturer           text                                                                NOT NULL,
    "profile_id"           uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE RESTRICT NOT NULL,
    public                 boolean                        DEFAULT true                         NOT NULL,
    "slicer_config"        text,
    "slicer_config_public" boolean                        DEFAULT true                         NOT NULL,
    description            text,
    "modified_scale"       "ModifiedScale"                DEFAULT 'NoMods'::"ModifiedScale"    NOT NULL
);


-- MANY TO MANY
CREATE TABLE "_Likes"
(
    profile_id uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES "Note" (id) ON UPDATE CASCADE,
    CONSTRAINT likes_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE "_Mentions"
(
    profile_id uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES "Note" (id) ON UPDATE CASCADE,
    CONSTRAINT mentions_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE "_Boosts"
(
    profile_id uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE CASCADE,
    note_id    uuid REFERENCES "Note" (id) ON UPDATE CASCADE,
    CONSTRAINT boosts_pkey PRIMARY KEY (profile_id, note_id)
);

CREATE TABLE "_Followers"
(
    profile_id  uuid REFERENCES "Profile" (id) ON UPDATE CASCADE ON DELETE CASCADE,
    follower_id uuid REFERENCES "Profile" (id) ON UPDATE CASCADE,
    CONSTRAINT followers_pkey PRIMARY KEY (profile_id, follower_id)
)