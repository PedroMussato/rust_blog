// @generated automatically by Diesel CLI.

diesel::table! {
    auth_user_session_tokens (session_token_id) {
        session_token_id -> Uuid,
        fk_user -> Uuid,
    }
}

diesel::table! {
    auth_users (user_id) {
        user_id -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        fullname -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        #[max_length = 512]
        password -> Varchar,
    }
}

diesel::joinable!(auth_user_session_tokens -> auth_users (fk_user));

diesel::allow_tables_to_appear_in_same_query!(
    auth_user_session_tokens,
    auth_users,
);
