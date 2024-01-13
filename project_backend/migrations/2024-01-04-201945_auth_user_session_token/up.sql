-- Your SQL goes here
CREATE TABLE auth_user_session_tokens (
    session_token_id UUID PRIMARY KEY,
    fk_user UUID REFERENCES auth_users(user_id) NOT NULL
);