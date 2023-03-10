use super::schema::{CreateInput, Password, UpdateInput};
use crate::config::mongo::get_col;
use crate::error::Result;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};

fn col_name() -> &'static str {
    "passman"
}

pub async fn create_password(input: CreateInput) -> Result<Password> {
    let mut doc = Password {
        _id: None,
        subject: input.subject,
        algo: input.algo,
        hash: input.hash,
        created_at: chrono::Utc::now(),
    };

    let col = get_col::<Password>(col_name());
    let data = col.insert_one(doc.clone(), None).await?;

    doc._id = data.inserted_id.as_object_id();

    Ok(doc)
}

pub async fn update(input: UpdateInput) -> Result<String> {
    let col = get_col::<Password>(col_name());
    let obj_id = ObjectId::parse_str(input._id).unwrap();
    let filter = doc! {"_id":obj_id};
    let update = doc! {"$set": {
      "subject": input.subject,
      "algo":input.algo.to_string(),
      "hash":input.hash,
    }};

    let _data = col.update_one(filter, update, None).await?;

    Ok("ok".to_owned())
}

pub async fn get_passwords() -> Result<Vec<Password>> {
    let col = get_col::<Password>(col_name());
    let cursor = col.find(None, None).await?;
    let data = cursor
        .try_collect::<Vec<Password>>()
        .await
        .unwrap_or(vec![]);
    Ok(data)
}

pub async fn password_detail(id: &String) -> Result<Password> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};

    let col = get_col::<Password>(col_name());

    let detail = col.find_one(filter, None).await?;

    Ok(detail.unwrap())
}
