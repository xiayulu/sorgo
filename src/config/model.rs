/// this model stores many constants
use crate::{
    config::mongo::get_col,
    error::{Error, Result},
};
use jwt_simple::prelude::*;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub key: String,
    pub value: String,
}

fn col_name() -> &'static str {
    "app_config"
}

pub async fn jwt_key_config() -> Result<Option<Config>> {
    let col = get_col::<Config>(col_name());
    let filter = doc! {"key":"jwt"};
    let key_bytes = ES256KeyPair::generate().to_bytes();
    let new = Config {
        _id: None,
        key: "jwt".to_owned(),
        value: hex::encode(key_bytes),
    };

    let update = doc! {"$setOnInsert":to_bson(&new).map_err(|e| {
        let e = format!("conver to bson:{}", e);
        log::error!("{}", e);
        Error::Value(e)
    })?};
    let options = FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(Some(ReturnDocument::After))
        .build();
    let result = col.find_one_and_update(filter, update, options).await?;

    Ok(result)
}
