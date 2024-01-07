/*
*#####################
* REGISTER
*#####################
*/

use axum::{extract::Json, response::Json as AxumJson};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use crate::functions::establish_connection;
use crate::functions::is_valid_email;
use crate::authentication::{Response, hash_sha512_256, create_user}; 
use crate::models::{NewAuthUsers, AuthUsers};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterPayload {
    username : String,
    fullname : String,
    email : String,
    password : String,
    password_confirmation : String,
}

pub async fn register(mut payload: Json<RegisterPayload>) -> AxumJson<Response> {
    use crate::schema::auth_users::dsl::*;

    
    payload.username = payload.username.to_lowercase();
    payload.email = payload.email.to_lowercase();
    
    let mut response = Response {
        is_ok : true,
        description : "".to_string() 
    };
    
    // Check if both passwords match
    if payload.password != payload.password_confirmation {
        response.is_ok = false;
        response.description += "-Password mismtach";
    } else {
        // Check if the string have 8 digits or more
        let char_count = payload.password.chars().count();        
        if char_count < 8 {
            response.is_ok = false;
            response.description += "-Password is too short, use passwords with 8 or more characters";
        }
        
        // Check if the password contains an UPPER CASE letter
        let contains_uppercase = !payload.password.chars().any(|c| c.is_ascii_uppercase());
        if contains_uppercase {
            response.is_ok = false;
            response.description += "-Password doesn't contains upper case letters";
        }
        
        // Check if the password contains an lower case letter
        let contains_lowercase = !payload.password.chars().any(|c| c.is_ascii_lowercase());
        if contains_lowercase {
            response.is_ok = false;
            response.description += "-Password doesn't contains lowe case letters";
        }
        
        // Check if the password contains an number letter
        let contains_numbers = !payload.password.chars().any(|c| c.is_digit(10));
        if contains_numbers {
            response.is_ok = false;
            response.description += "-Password doesn't contains numbers";
        }
    }
    
    // check if the username is valid
    if !payload.username.chars().all(|c| c.is_ascii_alphanumeric()) {
        response.is_ok = false;
        response.description += "-The username isn't valid, must contain only letters and numbers";
    }
    
    // check if the email is valid
    if !is_valid_email(&payload.email.to_string()) {
        response.is_ok = false;
        response.description += "-This email isn't valid";
    }
    
    let connection = &mut establish_connection();
    
    if response.is_ok != true {
        return AxumJson(response);
    } else  {
        // Check if a user with this username already exists
        let username_results = auth_users
            .filter(username.eq(&payload.username.to_string()))
            .select(AuthUsers::as_select())
            .load(connection)
            .expect("Error loading posts");
    
        if username_results.len() != 0 {
            response.is_ok = false;
            response.description += "-This username is already in use";
        }
        // Check if a user with this email already exists
        let email_results = auth_users
            .filter(email.eq(&payload.email.to_string()))
            .select(AuthUsers::as_select())
            .load(connection)
            .expect("Error loading posts");
        
        if email_results.len() != 0 {
            response.is_ok = false;
            response.description += "-This email is already in use";
        
        } 
    }
    
    if response.is_ok == true {
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