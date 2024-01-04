/*
 *########
 * MODULES
 *########
 */

 pub mod models;
 pub mod schema;


/*
 *#########################
 * DECLARATION OF LIBRARIES
 *#########################
 */
use axum::{extract::Json, response::Json as AxumJson};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::schema::auth_users::username;
use self::schema::auth_users::dsl::*;
//use self::models::*;
use uuid::Uuid;
use ring::digest::{Context, SHA512_256};
use regex::Regex;

//use diesel::prelude::*; 


/*
 *####################
 * DATABASE CONNECTION
 *####################
 */

use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


/*
 *####################
 * FUNCTIONS
 *####################
 */

fn hash_sha512_256(content: &str) -> String {
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

use self::models::{NewAuthUsers, AuthUsers};

pub fn create_user(conn: &mut PgConnection, new_user : NewAuthUsers) -> AuthUsers {

    let new_post = new_user;

    diesel::insert_into(auth_users)
        .values(&new_post)
        .returning(AuthUsers::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

fn is_valid_email(m: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+@[a-zA-Z0-9]+\.[a-zA-Z]+$").unwrap();
    re.is_match(m)
}

/*
 *#######################
 * DECLARATION OF STRUCTS
 *#######################
 */

 #[derive(Debug, Deserialize, Serialize)]
 pub struct RegisterPayload {
     username : String,
     fullname : String,
     email : String,
     password : String,
     password_confirmation : String,
 }
 
 #[derive(Debug, Deserialize, Serialize)]
 pub struct RegisterResponse{
     code : i32,
     description : String,
 }


/*
 *#####################
 * DECLARATION OF VIEWS
 *#####################
 */


pub async fn register(mut payload: Json<RegisterPayload>) -> AxumJson<RegisterResponse> {
    
    payload.username = payload.username.to_lowercase();
    payload.email = payload.email.to_lowercase();
    
    let mut response = RegisterResponse {
        code : 200,
        description : "".to_string() 
    };

    // Check if both passwords match
    if payload.password != payload.password_confirmation {
        response.code = 400;
        response.description += "-Password mismtach";
    } else {
        // Check if the string have 8 digits or more
        let char_count = payload.password.chars().count();        
        if char_count < 8 {
            response.code = 400;
            response.description += "-Password is too short, use passwords with 8 or more characters";
        }
        
        // Check if the password contains an UPPER CASE letter
        let contains_uppercase = !payload.password.chars().any(|c| c.is_ascii_uppercase());
        if contains_uppercase {
            response.code = 400;
            response.description += "-Password doesn't contains upper case letters";
        }

        // Check if the password contains an lower case letter
        let contains_lowercase = !payload.password.chars().any(|c| c.is_ascii_lowercase());
        if contains_lowercase {
            response.code = 400;
            response.description += "-Password doesn't contains lowe case letters";
        }

        // Check if the password contains an number letter
        let contains_numbers = !payload.password.chars().any(|c| c.is_digit(10));
        if contains_numbers {
            response.code = 400;
            response.description += "-Password doesn't contains numbers";
        }
    }

    // check if the username is valid
    if !payload.username.chars().all(|c| c.is_ascii_alphanumeric()) {
        response.code = 400;
        response.description += "-The username isn't valid, must contain only letters and numbers";
    }

    // check if the email is valid
    if !is_valid_email(&payload.email.to_string()) {
        response.code = 400;
        response.description += "-This email isn't valid";
    }

    let connection = &mut establish_connection();

    if response.code != 200 {
        return AxumJson(response);
    } else  {
        // Check if a user with this username already exists
        let username_results = auth_users
            .filter(username.eq(&payload.username.to_string()))
            .select(AuthUsers::as_select())
            .load(connection)
            .expect("Error loading posts");

        if username_results.len() != 0 {
            response.code = 400;
            response.description += "-This username is already in use";
        }
        // Check if a user with this email already exists
        let email_results = auth_users
            .filter(email.eq(&payload.email.to_string()))
            .select(AuthUsers::as_select())
            .load(connection)
            .expect("Error loading posts");

        if email_results.len() != 0 {
            response.code = 400;
            response.description += "-This email is already in use";

        } 
    }

    if response.code == 200 {
        let new_user = NewAuthUsers {
            id : &Uuid::new_v4(),
            username : &payload.username.to_string(),
            fullname : &payload.fullname.to_string(),
            email : &payload.email.to_string(),
            password : &hash_sha512_256(&payload.password.to_string()),    
        };
        create_user(connection, new_user);    
    }

    return AxumJson(response);
}


pub async fn login() -> impl IntoResponse {
    Html(format!("the endpoint /auth/login is working."))
}

pub async fn logout() -> impl IntoResponse {
    Html(format!("the endpoint /auth/logout is working."))
}

pub async fn redefine_password() -> impl IntoResponse {
    Html(format!("the endpoint /auth/redefine_password is working."))
}

pub async fn request_reset_password() -> impl IntoResponse {
    Html(format!("the endpoint /auth/request_reset_password is working."))
}

pub async fn reset_password() -> impl IntoResponse {
    Html(format!("the endpoint /auth/reset_password is working."))
}

pub async fn profile() -> impl IntoResponse {
    Html(format!("the endpoint /auth/profile is working."))
}

pub async fn update_profile() -> impl IntoResponse {
    Html(format!("the endpoint /auth/update_profile is working."))
}

pub async fn request_user_deletation() -> impl IntoResponse {
    Html(format!("the endpoint /auth/request_user_deletation is working."))
}

pub async fn delete_user() -> impl IntoResponse {
    Html(format!("the endpoint /auth/delete_user is working."))
}
