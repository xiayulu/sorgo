use std::fmt;

use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum Algo {
    AES,
}

impl fmt::Display for Algo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::AES => write!(f, "AES"),
        }
    }
}

//owner schema
#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct Password {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub subject: String,
    pub hash: String,
    pub algo: Algo,
    pub created_at: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct CreateInput {
    pub subject: String,
    pub hash: String,
    pub algo: Algo,
}

#[derive(InputObject)]
pub struct UpdateInput {
    pub _id: String,
    pub subject: String,
    pub hash: String,
    pub algo: Algo,
}

#[derive(InputObject)]
pub struct FetchPassword {
    pub _id: String,
}
