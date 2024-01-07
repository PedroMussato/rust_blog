-- Your SQL goes here
CREATE TABLE auth_user_session_tokens (
    id UUID PRIMARY KEY,
    fk_user UUID REFERENCES auth_users(id) NOT NULL
);