use super::model::jwt_key_config;
use jwt_simple::prelude::ES256KeyPair;
use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<ES256KeyPair> = OnceCell::new();

pub async fn init() {
    let config = jwt_key_config()
        .await
        .expect("get jwt key from database failed")
        .expect("Null jwt key");

    let key_pair =
        ES256KeyPair::from_bytes(&hex::decode(&config.value).expect("parse jwt key pair from hex"))
            .expect("parse jwt key pair failed");

    let _ = INSTANCE.set(key_pair);
}

pub fn get_pair() -> &'static ES256KeyPair {
    INSTANCE.get().expect("get jwt from pool")
}
