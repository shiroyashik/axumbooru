// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        creation_time -> Timestamp,
        last_edit_time -> Nullable<Timestamp>,
        #[max_length = 32]
        safety -> Varchar,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        #[max_length = 64]
        checksum -> Varchar,
        #[max_length = 2048]
        source -> Nullable<Varchar>,
        file_size -> Nullable<Int8>,
        image_width -> Nullable<Int4>,
        image_height -> Nullable<Int4>,
        #[sql_name = "mime-type"]
        #[max_length = 32]
        mime_type -> Varchar,
        version -> Int4,
        #[max_length = 32]
        flags -> Nullable<Varchar>,
        #[max_length = 32]
        checksum_md5 -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Nullable<Varchar>,
        #[max_length = 128]
        password -> Varchar,
        enabled -> Bool,
        #[max_length = 64]
        email -> Nullable<Varchar>,
        #[max_length = 32]
        access_level -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 32]
        avatar_style -> Varchar,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
