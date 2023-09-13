pub mod commands {
    use std::error::Error;

    pub async fn create_table_query(conn: &mut sqlx::PgConnection, tname: &str) -> Result<(), Box<dyn Error>> {
        let qry = 
                format!(
                    "CREATE TABLE IF NOT EXISTS {table_name}(
                    term VARCHAR (50) NOT NULL,
                    value VARCHAR (50) NOT NULL
                    );", table_name=tname);
        
        println!("{}", qry);

        sqlx::query(&qry)
            .execute(conn)
            .await?;
        
        dbg!("Creating table {name}", tname);

        Ok(())
    }
}