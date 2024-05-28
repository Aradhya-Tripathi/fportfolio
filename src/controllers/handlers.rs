use rocket::catch;
use rocket::http::Status;
use rocket::Request;
use serde_json::{json, Value};

#[catch(404)]
pub fn handle_404(request: &Request) -> (Status, Value) {
    let requested_path = request.uri();
    (
        Status::NotFound,
        json!({"error": format!("requested resource {requested_path} not found")}),
    )
}

#[catch(500)]
pub fn handle_500() -> (Status, Value) {
    (
        Status::InternalServerError,
        json!({"error": "server error!"}),
    )
}
