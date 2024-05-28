use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct Query {
    pub email: String,
    pub content: String,
}
