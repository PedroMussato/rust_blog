use axum::{extract::Json, response::Json as AxumJson};
use serde::Deserialize;
use serde::Serialize;
use crate::authentication::Response;
//use crate::functions::establish_connection;
//use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RedefinePasswordPayload {
    session_token : String,
    actual_password : String,
    password : String,
    password_confirmation : String,
}

pub async fn redefine_password(payload: Json<RedefinePasswordPayload>) -> AxumJson<Response> {
    let mut response = Response {
        is_ok : true,
        description : "".to_string()
    };

    // check if the password contains at least 8 digits, 1 number, 1 UPPER CASE letter and 1 lower case letter
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

    if response.is_ok {
        //let connection = establish_connection();

        //let token = Uuid::parse_str(&payload.session_token.to_string());

        // get the session token and the user id

        // get the user and compare if the password sent is equals the hash stored

        // if the password is equal change the user password hash to the hash of the password sent

        // else change response.is_ok to false and the description to "Actual password sent doesn't matchs with the user password."


    }

    AxumJson(response)
}

