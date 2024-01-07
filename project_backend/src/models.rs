use diesel::prelude::*;
use crate::schema::{auth_users, auth_user_session_tokens};
use uuid::Uuid;

////////////////////////////////////REGISTER//////////////////////////////////////

#[derive(Queryable, Selectable)]
#[diesel(table_name = auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUsers {
    pub id: Uuid,
    pub username: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
}


#[derive(Insertable)]
#[diesel(table_name = auth_users)]
pub struct NewAuthUsers<'a> {
    pub id: &'a Uuid,
    pub username: &'a String,
    pub fullname: &'a String,
    pub email: &'a String,
    pub password: &'a String,
}

///////////////////////////////////LOGIN///////////////////////////////////////

#[derive(Queryable, Selectable)]
#[diesel(table_name = auth_user_session_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUserSessionTokens {
    pub id: Uuid,
    pub fk_user: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = auth_user_session_tokens)]
pub struct NewAuthUserSessionTokens<'a> {
    pub id: &'a Uuid,
    pub fk_user: &'a Uuid,
}