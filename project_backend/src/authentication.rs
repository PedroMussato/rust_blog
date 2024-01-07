/*
 *#########################
 * DECLARATION OF LIBRARIES
 *#########################
 */
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use ring::digest::{Context, SHA512_256};
use diesel::pg::PgConnection;

/*
 *####################
 * FUNCTIONS
 *####################
 */
pub fn hash_sha512_256(content: &str) -> String {
    let mut context = Context::new(&SHA512_256);
    context.update(content.as_bytes());

    let digest = context.finish();
    // Convert the hash digest bytes to a hexadecimal string
    let mut hashed_password = String::new();
    for &byte in digest.as_ref() {
        hashed_password.push_str(&format!("{:02x}", byte));
    }

    hashed_password
}

use crate::models::{NewAuthUsers, AuthUsers, AuthUserSessionTokens, NewAuthUserSessionTokens};

pub fn create_user(conn: &mut PgConnection, new_user : NewAuthUsers) -> AuthUsers {
    use crate::schema::auth_users::dsl::*;

    diesel::insert_into(auth_users)
        .values(&new_user)
        .returning(AuthUsers::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn create_token(conn: &mut PgConnection, new_session_token : NewAuthUserSessionTokens) -> AuthUserSessionTokens {

    diesel::insert_into(crate::schema::auth_user_session_tokens::table)
        .values(&new_session_token)
        .returning(AuthUserSessionTokens::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}



/*
 *#######################
 * DECLARATION OF STRUCTS
 *#######################
 */

#[derive(Debug, Deserialize, Serialize)]
pub struct Response{
   pub(crate) is_ok : bool,
   pub(crate) description : String,
}  

//
//pub async fn redefine_password() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/redefine_password is working."))
//}
//
//pub async fn request_reset_password() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/request_reset_password is working."))
//}
//
//pub async fn reset_password() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/reset_password is working."))
//}
//
//pub async fn profile() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/profile is working."))
//}
//
//pub async fn update_profile() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/update_profile is working."))
//}
//
//pub async fn request_user_deletation() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/request_user_deletation is working."))
//}
//
//pub async fn delete_user() -> impl IntoResponse {
//    Html(format!("the endpoint /auth/delete_user is working."))
//}
