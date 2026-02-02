use diesel::{connection::SimpleConnection, prelude::*};

pub mod models;
pub mod queries;
pub mod schema;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(database_url).expect("Error connecting to database");

    // SQLite foreign keys are disabled by default and must be enabled per connection
    conn.batch_execute("PRAGMA foreign_keys = ON")
        .expect("Error enabling foreign keys");

    conn
}
