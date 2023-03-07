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
    var("MONGOURI").expect("APP_HOST not in .env");
}
