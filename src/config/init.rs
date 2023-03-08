use std::env::var;

use super::mongo;

pub async fn init() {
    dotenv::dotenv().expect("load .env file err");
    check();

    // init mongodb connect
    mongo::init().await;
}

pub fn check() {
    // app envs
    var("APP_HOST").expect("APP_HOST not in .env");
    var("APP_PORT")
        .expect("APP_PORT not in .env")
        .parse::<u16>()
        .expect("APP_PORT could not parse to u16");

    // database
    var("MONGO_URI").expect("APP_HOST not in .env");
}
