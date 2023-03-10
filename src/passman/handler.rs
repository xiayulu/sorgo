use super::model;
use super::schema::{CreateInput, FetchPassword, Password, UpdateInput};
use actix_web::{guard, web};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

pub struct Query;

#[Object(extends)]
impl Query {
    async fn password(&self, _ctx: &Context<'_>, input: FetchPassword) -> FieldResult<Password> {
        let data = model::password_detail(&input._id).await.unwrap();
        Ok(data)
    }

    async fn get_passwords(&self, _ctx: &Context<'_>) -> FieldResult<Vec<Password>> {
        let data = model::get_passwords().await.unwrap();
        Ok(data)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_password(
        &self,
        _ctx: &Context<'_>,
        input: CreateInput,
    ) -> FieldResult<Password> {
        let data = model::create_password(input).await.unwrap();
        Ok(data)
    }

    async fn update(&self, _ctx: &Context<'_>, input: UpdateInput) -> FieldResult<String> {
        let data = model::update(input).await.unwrap();
        Ok(data)
    }
}

// http handler
async fn index(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    schema.execute(req.into_inner()).await.into()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/passman").guard(guard::Post()).to(index));
}
