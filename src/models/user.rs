use crate::database::DbConn;
use crate::forms;

use crate::schema::users;

use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub encrypted_password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub encrypted_password: String,
}

impl User {
    pub fn find(db_conn: DbConn, id: i32)
        -> Result<Self, diesel::result::Error>
    {
        let query = users::table.find(id);

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.first::<Self>(&*db_conn)
    }

    pub fn by_username(db_conn: DbConn, username: String)
        -> Result<Self, diesel::result::Error>
    {
        let query = users::table.filter(users::username.eq(username));

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.first::<Self>(&*db_conn)
    }

    pub fn all(db_conn: DbConn) -> Result<Vec<Self>, diesel::result::Error> {
        let query = users::table.as_query();

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.load::<Self>(&*db_conn)
    }

    pub fn authorize(self, password: &String) -> bool {
        match bcrypt::verify(password, self.encrypted_password.as_str()) {
            Err(_) => false,
            Ok(value) => value,
        }
    }
}

impl NewUser {
    pub fn from_form(form: forms::UserSignUp) -> Result<Self, ValidationErrors> {
        form.validate()?;

        let encrypted_password = bcrypt::hash(
            form.password.to_string(),
            bcrypt::DEFAULT_COST,
        ).unwrap();

        Ok(Self {
            username: form.username,
            encrypted_password: encrypted_password,
        })
    }

    pub fn save(&self, db_conn: DbConn) -> Result<(), diesel::result::Error> {
        let query = diesel::insert_into(users::table).values(self);

        let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);

        println!("{}", debug);

        query.get_result::<User>(&*db_conn)?;

        Ok(())
    }
}
