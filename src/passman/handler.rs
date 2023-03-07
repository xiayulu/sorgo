use super::schema::{CreateInput, FetchPassword, Password, UpdateInput};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::model;

pub struct Query;

#[Object(extends)]
impl Query {
    //owners query
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
pub async fn index(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    schema.execute(req.into_inner()).await.into()
}
