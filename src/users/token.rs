use crate::error::Result;
use crate::{config::jwt::get_pair, error::Error};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Token {
    user_id: String,
}

impl Token {
    pub fn builder() -> Self {
        Self {
            user_id: String::from(""),
        }
    }

    pub fn set_user_id(mut self, user_id: &str) -> Self {
        self.user_id = String::from(user_id);
        self
    }

    pub fn create_jwt(self) -> Result<String> {
        let key_pair = get_pair();
        let claims = Claims::with_custom_claims(self, Duration::from_days(7));
        Ok(key_pair.sign(claims).map_err(|e| {
            let e = format!("create jwt token failed:{}", e);
            log::error!("{}", e);
            Error::Server(e)
        })?)
    }

    pub fn verify_jwt(token: &str) -> Result<Token> {
        let key_pair = get_pair();
        let public_key = key_pair.public_key();

        let claims = public_key
            .verify_token::<Token>(&token, None)
            .map_err(|e| {
                let e = format!("create jwt token failed:{}", e);
                log::error!("{}", e);
                Error::Auth
            })?;
        Ok(claims.custom)
    }
}
