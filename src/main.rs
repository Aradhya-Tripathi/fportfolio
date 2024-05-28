use fportfolio::controllers;
use fportfolio::models::projects;
use rocket::{self, catchers};

#[rocket::main]
async fn main() {
    rocket::build()
        .manage(projects::Project::get())
        .register(
            "/",
            catchers![
                controllers::handlers::handle_404,
                controllers::handlers::handle_500,
                controllers::handlers::handle_422
            ],
        )
        .mount(
            "/",
            rocket::routes![
                controllers::portfolio::projects,
                controllers::portfolio::query,
                controllers::portfolio::health,
                controllers::portfolio::summarize_project, // Hover support.
            ],
        )
        .launch()
        .await
        .unwrap();
}
