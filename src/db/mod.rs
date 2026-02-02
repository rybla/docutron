use crate::config::DATABASE_URL;
use diesel::{connection::SimpleConnection, prelude::*};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};

#[cfg(test)]
use diesel_migrations::MigrationHarness;

pub mod models;
pub mod queries;
pub mod schema;

#[cfg(test)]
mod test;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn establish_connection() -> SqliteConnection {
    establish_common_connection(&DATABASE_URL)
}

#[cfg(test)]
pub fn establish_test_connection() -> SqliteConnection {
    let mut conn = establish_common_connection(":memory:");
    MigrationHarness::run_pending_migrations(&mut conn, MIGRATIONS)
        .expect("Error running migrations");
    conn
}

pub fn establish_common_connection(database_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(database_url).expect("Error connecting to database");

    // SQLite foreign keys are disabled by default and must be enabled per connection
    conn.batch_execute("PRAGMA foreign_keys = ON")
        .expect("Error enabling foreign keys");

    conn
}
