use crate::models::Describe;
use rocket::serde::{Deserialize, Serialize};

use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Project {
    pub id: u8,
    pub name: String,
    pub link: String,
    pub description: String,
    pub tags: Vec<String>,
    pub image: String,
    pub technologies: Vec<String>,
}

impl Project {
    pub fn get() -> Vec<Project> {
        let file_handler = fs::File::open("projects.json").expect("Unable to find projects!");
        serde_json::from_reader(file_handler).expect("Badly formatted projects file!")
    }
}

impl Describe for Project {
    fn describe(&self) -> String {
        format!(
            "{name} uses the following technologies {technologies}",
            name = self.name,
            technologies = self.technologies.join(" ")
        )
    }
}
