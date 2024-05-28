pub mod projects;
pub mod query;

pub trait Describe {
    fn describe(&self) -> String;
}
