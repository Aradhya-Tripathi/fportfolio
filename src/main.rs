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
                controllers::handlers::handle_500
            ],
        )
        .mount(
            "/",
            rocket::routes![
                controllers::portfolio::index,
                controllers::portfolio::projects,
                controllers::portfolio::query,
                controllers::portfolio::health,
                controllers::portfolio::summarize_projects,
            ],
        )
        .launch()
        .await
        .unwrap();
}
