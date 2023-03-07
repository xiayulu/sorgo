//Owners logiuse dotenv::dotenv;
use super::schema::{Owner, Project};
use crate::config::mongo::get_col;
use futures::TryStreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use std::io::Error;

pub async fn create_owner(new_owner: Owner) -> Result<Owner, Error> {
    let new_doc = Owner {
        _id: None,
        name: new_owner.name.clone(),
        email: new_owner.email.clone(),
        phone: new_owner.phone.clone(),
    };

    let col = get_col::<Owner>("owner");

    let data = col
        .insert_one(new_doc, None)
        .await
        .expect("Error creating owner");

    let new_owner = Owner {
        _id: data.inserted_id.as_object_id(),
        name: new_owner.name.clone(),
        email: new_owner.email.clone(),
        phone: new_owner.phone.clone(),
    };

    Ok(new_owner)
}

pub async fn get_owners() -> Result<Vec<Owner>, Error> {
    let col = get_col::<Owner>("owner");

    let mut cursors = col
        .find(None, None)
        .await
        .expect("Error getting list of owners");

    let mut owners: Vec<Owner> = Vec::new();
    while let Some(owner) = cursors
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        owners.push(owner)
    }

    Ok(owners)
}

pub async fn single_owner(id: &String) -> Result<Owner, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};

    let col = get_col::<Owner>("owner");

    let owner_detail = col
        .find_one(filter, None)
        .await
        .expect("Error getting owner's detail");

    Ok(owner_detail.unwrap())
}

//project logics
pub async fn create_project(new_project: Project) -> Result<Project, Error> {
    let new_doc = Project {
        _id: None,
        owner_id: new_project.owner_id.clone(),
        name: new_project.name.clone(),
        description: new_project.description.clone(),
        status: new_project.status.clone(),
    };

    let col = get_col::<Project>("project");

    let data = col
        .insert_one(new_doc, None)
        .await
        .expect("Error creating project");

    let new_project = Project {
        _id: data.inserted_id.as_object_id(),
        owner_id: new_project.owner_id.clone(),
        name: new_project.name.clone(),
        description: new_project.description.clone(),
        status: new_project.status.clone(),
    };

    Ok(new_project)
}

pub async fn get_projects() -> Result<Vec<Project>, Error> {
    let col = get_col::<Project>("project");

    let mut cursors = col
        .find(None, None)
        .await
        .expect("Error getting list of projects");

    let mut projects: Vec<Project> = Vec::new();
    while let Some(project) = cursors
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        projects.push(project)
    }

    Ok(projects)
}

pub async fn single_project(id: &String) -> Result<Project, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};

    let col = get_col::<Project>("project");

    let project_detail = col
        .find_one(filter, None)
        .await
        .expect("Error getting project's detail");

    Ok(project_detail.unwrap())
}
