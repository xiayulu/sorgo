use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

//project schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Mark {
    Normal = 0,
    Blocked = -1,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub mark: Mark,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct SigninRes {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Count {
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct List {
    pub users: Vec<User>,
    pub total: [Count; 1],
}

// simple ok message
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Msg {
    pub msg: String,
}

#[derive(InputObject)]
pub struct UserQuery {
    pub page: i32,
    pub page_size: i32,
}

#[derive(InputObject)]
pub struct EmailInput {
    pub email: String,
}

#[derive(InputObject)]
pub struct SignInInput {
    pub email: String,
    pub captcha: String,
}

#[derive(InputObject)]
pub struct UserID {
    pub _id: String,
}
