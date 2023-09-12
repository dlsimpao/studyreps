use std::env;
pub fn get_creds(){
    let user = env::var("PG_USER").unwrap();
    let pass = env::var("PG_PASS").unwrap();
    let dbnm = env::var("PG_DBNM").unwrap();
}

pub fn auth_pg(){
    println!("Authenticating!")

}