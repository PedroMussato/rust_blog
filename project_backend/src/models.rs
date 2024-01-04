use diesel::prelude::*;
use crate::schema::auth_users;
use uuid::Uuid;

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