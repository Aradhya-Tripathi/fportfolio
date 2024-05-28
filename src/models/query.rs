use crate::models::Describe;
use rocket::serde::{Deserialize, Serialize};

use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub email: String,
    pub content: String,
}

impl Query {
    pub fn save(&self) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("query.txt")
            .map_err(|err| format!("Failed to open file: {}", err))?;

        match file.write_all(self.describe().as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err("Failed to write to file!".to_string()),
        }
    }
}

impl Describe for Query {
    fn describe(&self) -> String {
        format!(
            "{email}: {content}\n",
            email = self.email,
            content = self.content
        )
    }
}
