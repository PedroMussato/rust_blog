// @generated automatically by Diesel CLI.

diesel::table! {
    auth_users (id) {
        id -> Uuid,
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
