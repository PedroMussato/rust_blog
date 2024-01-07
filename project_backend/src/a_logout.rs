/*
 *#####################
 * LOGOUT
 *#####################
 */

use axum::{extract::Json, response::Json as AxumJson};
use serde::Deserialize;
use serde::Serialize;
use crate::authentication::Response;
use uuid::Uuid;

use diesel::prelude::*;
use diesel::delete;
use crate::functions::establish_connection;

use crate::schema::auth_user_session_tokens::dsl::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct LogoutPayload {
    session_token : String,
}

pub async fn logout(payload: Json<LogoutPayload>) -> AxumJson<Response> {

    let connection = &mut establish_connection();
    
    let token = Uuid::parse_str(&payload.session_token.to_string());

    let num_deleted = delete(auth_user_session_tokens
        .filter(id.eq_any(token)))
        .execute(connection)
        .expect("Error loading user");
    
    // declaration of response object
    let response = Response {
        is_ok : true,
        description : format!("{}",num_deleted).to_string(),
    };
    AxumJson(response)
}