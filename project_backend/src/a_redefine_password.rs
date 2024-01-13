use axum::{extract::Json, response::Json as AxumJson};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use serde::Deserialize;
use serde::Serialize;
use crate::authentication::Response;
use crate::authentication::check_password_complexity;
use crate::authentication::get_user_by_session_token;
use crate::authentication::hash_sha512_256;
use diesel::RunQueryDsl;


#[derive(Debug, Deserialize, Serialize)]
pub struct RedefinePasswordPayload {
    session_token : String,
    actual_password : String,
    password : String,
    password_confirmation : String,
}

pub async fn redefine_password(payload: Json<RedefinePasswordPayload>) -> AxumJson<Response> {    

    let mut response = check_password_complexity(payload.password.to_string(), payload.password_confirmation.to_string());

    if response.is_ok {
        // get the session token and the user id
        let user = get_user_by_session_token(payload.session_token.to_string());
        
        // get the user and compare if the password sent is equals the hash stored
        let hashed_actual_password = hash_sha512_256(&payload.actual_password);

        
        // ... rest of your code

        // if the password is equal change the user password hash to the hash of the password sent
        if user.password == hashed_actual_password {
            diesel::update(crate::schema::auth_users::dsl::auth_users.find(user.user_id))
                .set(crate::schema::auth_users::dsl::password.eq(hash_sha512_256(&payload.password)))
                .execute(&mut crate::authentication::establish_connection())
                .expect("Error loading user");
        } else{
            response.is_ok = false;
            response.description = "Actual password sent doesn't matchs with the user password.".to_string();
        }        
    }

    AxumJson(response)
}

