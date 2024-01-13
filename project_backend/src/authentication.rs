/*
 *#########################
 * DECLARATION OF LIBRARIES
 *#########################
 */
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use ring::digest::{Context, SHA512_256};
use diesel::pg::PgConnection;
use uuid::Uuid;
use diesel::Connection;
use regex::Regex;

//use crate::functions::establish_connection;

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

pub fn get_user_by_session_token(token : String) -> AuthUsers {
    let mut connection = crate::authentication::establish_connection();
    let token_id = Uuid::parse_str(&token);
    
    let session_token_registry = crate::schema::auth_user_session_tokens::dsl::auth_user_session_tokens
        .filter(crate::schema::auth_user_session_tokens::dsl::session_token_id.eq_any(token_id))
        .select(crate::schema::auth_user_session_tokens::dsl::fk_user)
        .load::<Uuid>(&mut connection)
        .expect("Error loading user");

    let user_registry = crate::schema::auth_users::dsl::auth_users
        .filter(crate::schema::auth_users::dsl::user_id.eq(&session_token_registry.first().unwrap()))
        .select(AuthUsers::as_select())
        .load(&mut connection)
        .expect("Error loading user");

    AuthUsers {
        user_id : user_registry.first().unwrap().user_id,
        username : user_registry.first().unwrap().username.to_string(),
        fullname : user_registry.first().unwrap().fullname.to_string(),
        email : user_registry.first().unwrap().email.to_string(),
        password : user_registry.first().unwrap().password.to_string(),
    }
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

pub fn check_password_complexity(password : String, password_confirmation : String) -> Response {
    let mut response = Response {
        is_ok : true,
        description : "".to_string()
    };

    if password != password_confirmation {
        response.is_ok = false;
        response.description += "-Password mismtach";
    } else {
        // Check if the string have 8 digits or more
        let char_count = password.chars().count();
        if char_count < 8 {
            response.is_ok = false;
            response.description += "-Password is too short, use passwords with 8 or more characters";
        }

        // Check if the password contains an UPPER CASE letter
        let contains_uppercase = !password.chars().any(|c| c.is_ascii_uppercase());
        if contains_uppercase {
            response.is_ok = false;
            response.description += "-Password doesn't contains upper case letters";
        }

        // Check if the password contains an lower case letter
        let contains_lowercase = !password.chars().any(|c| c.is_ascii_lowercase());
        if contains_lowercase {
            response.is_ok = false;
            response.description += "-Password doesn't contains lowe case letters";
        }

        // Check if the password contains an number letter
        let contains_numbers = !password.chars().any(|c| c.is_digit(10));
        if contains_numbers {
            response.is_ok = false;
            response.description += "-Password doesn't contains numbers";
        }
    }
    response
}


pub fn establish_connection() -> PgConnection {
    use dotenvy::dotenv;
    use std::env;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn is_valid_email(m: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+@[a-zA-Z0-9]+\.[a-zA-Z]+$").unwrap();
    re.is_match(m)
}

pub fn is_valid_password(m: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+@[a-zA-Z0-9]+\.[a-zA-Z]+$").unwrap();
    re.is_match(m)

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
