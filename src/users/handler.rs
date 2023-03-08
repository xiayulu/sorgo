use super::schema::{CreateOwner, CreateProject, FetchOwner, FetchProject, Owner, Project};
use actix_web::{guard, web};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::model;

pub struct Query;

#[Object(extends)]
impl Query {
    //owners query
    async fn owner(&self, _ctx: &Context<'_>, input: FetchOwner) -> FieldResult<Owner> {
        let owner = model::single_owner(&input._id).await.unwrap();
        Ok(owner)
    }

    async fn get_owners(&self, _ctx: &Context<'_>) -> FieldResult<Vec<Owner>> {
        let owners = model::get_owners().await.unwrap();
        Ok(owners)
    }

    //projects query
    async fn project(&self, _ctx: &Context<'_>, input: FetchProject) -> FieldResult<Project> {
        let project = model::single_project(&input._id).await.unwrap();
        Ok(project)
    }

    async fn get_projects(&self, _ctx: &Context<'_>) -> FieldResult<Vec<Project>> {
        let projects = model::get_projects().await.unwrap();
        Ok(projects)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    //owner mutation
    async fn create_owner(&self, _ctx: &Context<'_>, input: CreateOwner) -> FieldResult<Owner> {
        let new_owner = Owner {
            _id: None,
            email: input.email,
            name: input.name,
            phone: input.phone,
        };
        let owner = model::create_owner(new_owner).await.unwrap();
        Ok(owner)
    }

    async fn create_project(
        &self,
        _ctx: &Context<'_>,
        input: CreateProject,
    ) -> FieldResult<Project> {
        let new_project = Project {
            _id: None,
            owner_id: input.owner_id,
            name: input.name,
            description: input.description,
            status: input.status,
        };
        let project = model::create_project(new_project).await.unwrap();
        Ok(project)
    }
}

// http handler
async fn index(req: GraphQLRequest) -> GraphQLResponse {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    schema.execute(req.into_inner()).await.into()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").guard(guard::Post()).to(index));
}
