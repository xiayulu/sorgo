use super::schema::{List, Mark, User, UserQuery};
use super::token::MyClaims;
use crate::config::mongo::get_col;
use crate::error::{Error, Result};
use actix_web::http::header::HeaderMap;
use actix_web::HttpRequest;
use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::{self, doc, oid::ObjectId, to_bson};
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

pub async fn get_user_by_id(id: &str) -> Result<Option<User>> {
    let col = get_col::<User>(col_name());
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id":obj_id};

    let result = col.find_one(filter, None).await?;

    Ok(result)
}

pub async fn get_user_list(query: UserQuery) -> Result<Option<List>> {
    let col = get_col::<User>(col_name());
    let filter = doc! {};
    let stage = doc! { "$facet": {
      "users": [
        { "$match": filter},
        { "$skip": (query.page-1)*query.page_size },
        { "$limit": query.page_size }
      ],
      "total": [{ "$count": "count" }],
    }};

    let mut cursor = col.aggregate([stage], None).await?;

    while let Some(doc) = cursor.next().await {
        let data: List = bson::from_document(doc?).map_err(|e| {
            let e = format!("parse user list: {}", e);
            log::error!("{}", e);
            Error::Value(e)
        })?;
        return Ok(Some(data));
    }
    Ok(None)
}

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Token")
        .and_then(|value| value.to_str().map(|s| s.to_string()).ok())
}

pub async fn auth(req: HttpRequest) -> Result<Option<User>> {
    if let Some(token) = get_token_from_headers(req.headers()) {
        let claim = MyClaims::verify_jwt(&token)?;
        Ok(get_user_by_id(&claim.user_id).await?)
    } else {
        Ok(None)
    }
}
