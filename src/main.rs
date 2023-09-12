use std::error::Error;
use dotenv::dotenv;
use sqlx::Connection;
use sqlx::Row;
use std::env;
use crate::pg_auth;

pub mod auth;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let user = env::var("PG_USER").unwrap();
    let pass = env::var("PG_PASS").unwrap();
    let dbnm = env::var("PG_DBNM").unwrap();
    let url = format!("postgres://{user}:{pass}@localhost:5432/{database}", user=user, pass=pass, database=dbnm); //hello-postgres

    let args: Vec<String> = env::args().collect();

    println!("{}",url);
    let mut conn = sqlx::postgres::PgConnection::connect(&url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await?;

    let sum:i32 = res.get("sum");

    println!("1 + 1 = {}", sum);
    Ok(())
}