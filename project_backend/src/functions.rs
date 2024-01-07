use diesel::{PgConnection, Connection};
use regex::Regex;

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