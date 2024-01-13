use axum::{extract::Json, response::Json as AxumJson};
use diesel::{ExpressionMethods, BoolExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::authentication::{hash_sha512_256, Response, create_token, establish_connection};
use crate::models::{NewAuthUserSessionTokens, AuthUsers};
use crate::schema::auth_users::dsl::*;

//use diesel::query_dsl::methods::{FilterDsl, LoadQuery};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    username_or_email : String,
    password : String,
}

pub async fn login(payload: Json<LoginPayload>) -> AxumJson<Response> {
    // declaration of response object
    let mut response = Response {
        is_ok : true,
        description : "".to_string(),
    };

    let hash_password_sent = hash_sha512_256(&payload.password.to_string());

    let mut connection = establish_connection();

    let users = auth_users
        .filter(username.eq(&payload.username_or_email)
        .or(email.eq(&payload.username_or_email)))
        .select(AuthUsers::as_select())
        .load(&mut connection)
        .expect("Error loading posts");
    
    if let Some(user) = users.first() {
        // Compare the hashed password from the database with the hashed password sent during login
        if user.password == hash_password_sent {
            // Passwords match, login successful
            let new_token = NewAuthUserSessionTokens {
                session_token_id : &Uuid::new_v4(),
                fk_user : & user.user_id,
            };

            let token = create_token(&mut connection, new_token);

            response = Response {
                is_ok: true,
                description: format!("{}", token.session_token_id),
            };

        } else {
            response = Response {
                is_ok: false,
                description: "Invalid login informations".to_string(),
            };
        }
    } else {
        response = Response {
            is_ok: false,
            description: "Invalid login informations".to_string(),
        };
    }

    return AxumJson(response);
}
