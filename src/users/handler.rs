use super::email;
use super::model;
use super::schema::SignInInput;
use super::schema::SigninRes;
use super::schema::{EmailInput, Msg, User};
use super::token::Token;
use crate::config::redis;
use actix_web::{guard, web};
use async_graphql::{Context, EmptySubscription, Error, FieldResult, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub struct Query;

fn captcha_prefix() -> &'static str {
    "/user/captcha"
}

#[Object(extends)]
impl Query {
    async fn send_captcha(&self, _ctx: &Context<'_>, input: EmailInput) -> FieldResult<Msg> {
        let c = email::get_captcha();
        // save to redis
        redis::set_ex(format!("{}/{}", captcha_prefix(), input.email), &c, 60 * 10).await?;

        email::send_captcha(&c, &input.email)?;
        Ok(Msg {
            msg: "ok".to_owned(),
        })
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signin(&self, _ctx: &Context<'_>, input: SignInInput) -> FieldResult<SigninRes> {
        let captcha: String = redis::get(format!("{}/{}", captcha_prefix(), &input.email)).await?;
        if captcha != input.captcha {
            return Err(Error::new("邮箱或验证码错误"));
        }

        let user = model::signin(&input.email).await?;
        if user.is_none() {
            return Err(Error::new("signin failed: null user"));
        }

        let user = user.unwrap();
        if user._id.is_none() {
            return Err(Error::new("signin failed: null _id"));
        }

        let user_id = &user._id;
        let token = Token::builder().set_user_id(&user_id.unwrap().to_hex());
        Ok(SigninRes {
            user,
            token: token.create_jwt()?,
        })
    }
}

// http handler
async fn index(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    schema.execute(req.into_inner()).await.into()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").guard(guard::Post()).to(index));
}
