use anyhow::Result;
use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&database_url)?)
}
