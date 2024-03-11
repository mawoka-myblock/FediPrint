// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "EventAudience"))]
    pub struct EventAudience;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ModifiedScale"))]
    pub struct ModifiedScale;
}

diesel::table! {
    Account (id) {
        id -> Uuid,
        registered_at -> Timestamp,
        updated_at -> Timestamp,
        password -> Text,
        email -> Text,
        verified -> Nullable<Text>,
        profile_id -> Uuid,
        private_key -> Text,
    }
}

diesel::table! {
    File (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        mime_type -> Text,
        size -> Int8,
        file_name -> Nullable<Text>,
        description -> Nullable<Text>,
        alt_text -> Nullable<Text>,
        thumbhash -> Nullable<Text>,
        preview_file_id -> Nullable<Uuid>,
        to_be_deleted_at -> Nullable<Timestamp>,
        profile_id -> Uuid,
        file_for_model_id -> Nullable<Uuid>,
        image_for_model_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    Model (id) {
        id -> Uuid,
        server -> Text,
        server_id -> Nullable<Text>,
        profile_id -> Uuid,
        published -> Bool,
        title -> Text,
        summary -> Text,
        description -> Text,
        tags -> Array<Nullable<Text>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EventAudience;

    Note (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        server_id -> Nullable<Text>,
        content -> Text,
        hashtags -> Array<Nullable<Text>>,
        audience -> EventAudience,
        in_reply_to_comment_id -> Nullable<Uuid>,
        in_reply_to_note_id -> Nullable<Uuid>,
        actor_id -> Uuid,
        comment_of_model_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ModifiedScale;

    Printer (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Text,
        manufacturer -> Text,
        profile_id -> Uuid,
        public -> Bool,
        slicer_config -> Nullable<Text>,
        slicer_config_public -> Bool,
        description -> Nullable<Text>,
        modified_scale -> ModifiedScale,
    }
}

diesel::table! {
    Profile (id) {
        id -> Uuid,
        username -> Text,
        server -> Text,
        server_id -> Text,
        display_name -> Text,
        summary -> Text,
        inbox -> Text,
        outbox -> Text,
        public_key -> Text,
        registered_at -> Date,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    _Boosts (profile_id, note_id) {
        profile_id -> Uuid,
        note_id -> Uuid,
    }
}

diesel::table! {
    _Followers (profile_id, follower_id) {
        profile_id -> Uuid,
        follower_id -> Uuid,
    }
}

diesel::table! {
    _Likes (profile_id, note_id) {
        profile_id -> Uuid,
        note_id -> Uuid,
    }
}

diesel::table! {
    _Mentions (profile_id, note_id) {
        profile_id -> Uuid,
        note_id -> Uuid,
    }
}

diesel::joinable!(Account -> Profile (profile_id));
diesel::joinable!(File -> Profile (profile_id));
diesel::joinable!(Model -> Profile (profile_id));
diesel::joinable!(Note -> Model (comment_of_model_id));
diesel::joinable!(Note -> Profile (actor_id));
diesel::joinable!(Printer -> Profile (profile_id));
diesel::joinable!(_Boosts -> Note (note_id));
diesel::joinable!(_Boosts -> Profile (profile_id));
diesel::joinable!(_Likes -> Note (note_id));
diesel::joinable!(_Likes -> Profile (profile_id));
diesel::joinable!(_Mentions -> Note (note_id));
diesel::joinable!(_Mentions -> Profile (profile_id));

diesel::allow_tables_to_appear_in_same_query!(
    Account, File, Model, Note, Printer, Profile, _Boosts, _Followers, _Likes, _Mentions,
);
