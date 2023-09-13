pub mod pg_auth{

    pub use std::env;

    pub fn get_pg_connection() -> String{
        let user = env::var("PG_USER").unwrap();
        let pass = env::var("PG_PASS").unwrap();
        let dbnm = env::var("PG_DBNM").unwrap();
        
        let url = format!("postgres://{user}:{pass}@localhost:5432/{database}", user=user, pass=pass, database=dbnm);
        url
    }
}

