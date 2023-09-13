use std::error::Error;
use dotenv::dotenv;
use sqlx::Connection;
use sqlx::Row;
//use std::env;

use postgres::pgsql;
pub mod postgres;




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let conn_string = postgres::auth::pg_auth::get_pg_connection();

    //let args: Vec<String> = env::args().collect();

    println!("{}",conn_string);
    let mut conn = sqlx::postgres::PgConnection::connect(&conn_string).await?;

    pgsql::commands::create_table_query(&mut conn, "hello_world2").await?;

    // let res = sqlx::query(&qry)
    //     .fetch_one(&mut conn)
    //     .await?;

    Ok(())
}