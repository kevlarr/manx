use chrono::Utc;
use diesel:: prelude::*;
use diesel::pg::PgConnection;

use crate::schema;
use crate::models::{User, NewUser};
use super::StoreError;

type StoreResult<T> = std::result::Result<T, StoreError>;

pub fn new(email: String, password: String) -> NewUser {
    NewUser { created: Utc::now(), email, password }
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> StoreResult<User> {
    diesel::insert_into(schema::users::table)
        .values(new_user)
        .get_result(conn)
        .map_err(|e| StoreError(format!("{}", e)))
}

pub fn find_by_email(conn: &PgConnection, email: &str) -> Option<User> {
    use schema::users::dsl::{users, email as email_};

    users.filter(email_.eq(email)).first(conn).ok()
}

/*
pub struct Query {
    email: Option<String>,
    password: Option<String>,
}

impl Query {
    pub fn email(&mut self, email: &str) -> &mut Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    pub fn execute(&self, conn: &PgConnection) -> Option<User> {
        use schema::users::dsl as dsl;
        let mut q = dsl::users.into_boxed();

        if let Some(e) = &self.email {
            println!("email: {}", e);
            q = q.filter(dsl::email.eq(e));
        }

        if let Some(p) = &self.password {
            println!("password: {}", p);
            q = q.filter(dsl::password.eq(p));
        }

        q.first(conn).ok()
    }
}

pub fn query() -> Query {
    Query { email: None, password: None }
}
*/
