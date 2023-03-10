use std::env::var;

use super::jwt;
use super::mongo;
use super::redis;

pub async fn init() {
    dotenv::dotenv().expect("load .env file err");
    check();

    // init mongodb connect
    mongo::init().await;
    redis::init();
    jwt::init().await;
}

pub fn check() {
    // app envs
    var("APP_HOST").expect("APP_HOST not in .env");
    var("APP_PORT")
        .expect("APP_PORT not in .env")
        .parse::<u16>()
        .expect("APP_PORT could not parse to u16");

    // database
    var("MONGO_URI").expect("MONGO_URI not in .env");
    var("APP_DB").expect("APP_DB not in .env");

    // redis
    var("REDIS_URL").expect("REDIS_URL not in .env");

    // email
    var("EMAIL_HOST_USER").expect("EMAIL_HOST_USER not in .env");
    var("EMAIL_HOST_PASSWORD").expect("EMAIL_HOST_PASSWORD not in .env");
    var("EMAIL_HOST").expect("EMAIL_HOST not in .env");
    var("EMAIL_PORT").expect("EMAIL_PORT not in .env");
}
