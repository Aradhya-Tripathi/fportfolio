use crate::models::projects::Project;
use crate::models::query::Query;
use crate::models::Describe;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, State};
use serde_json::{json, Value};

#[get("/")]
pub fn index() -> Value {
    json!({
        "page": "Index Page",
    })
}

#[get("/projects")]
pub fn projects(projects: &State<Vec<Project>>) -> (Status, Json<Vec<Project>>) {
    (Status::Ok, Json(projects.to_vec()))
}

#[get("/summarize/project/<id>")]
pub fn summarize_projects(id: u8, projects: &State<Vec<Project>>) -> (Status, Value) {
    for project in projects.iter() {
        if project.id == id {
            return (
                Status::Ok,
                json!({
                    "description": project.describe()
                }),
            );
        }
    }
    (
        Status::BadRequest,
        json!({
            "error": "Invalid project Id"
        }),
    )
}

#[post("/query", data = "<query>")]
pub fn query(query: Json<Query>) -> (Status, Json<Query>) {
    (Status::Ok, query)
}

#[get("/health")]
pub fn health() -> Status {
    Status::ImATeapot
}
