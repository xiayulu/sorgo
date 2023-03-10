use super::schema::{Mark, User};
use crate::config::mongo::get_col;
use crate::error::{Error, Result};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId, to_bson};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use nanoid::nanoid;

fn col_name() -> &'static str {
    "user"
}

pub async fn signin(email: &str) -> Result<Option<User>> {
    let col = get_col::<User>(col_name());
    let filter = doc! {"emial":&email};
    let name = nanoid!(10);
    let avatar = format!("https://api.multiavatar.com/{}.png", name);
    let new_user = User {
        _id: None,
        name,
        avatar,
        email: email.to_owned(),
        mark: Mark::Normal,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let update = doc! {"$setOnInsert":to_bson(&new_user).map_err(|e| {
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
