use rusqlite::Result;

use crate::dots::{
    self,
    project::{self, Project},
};

pub fn lists() -> Vec<Project> {
    let projects = project::get_projects();
    projects
}

pub fn info(project_name: &String) -> Option<Project> {
    project::get_project(project_name)
}

pub fn delete(id: &u32) -> Vec<Project> {
    let borrow_id = &id;
    project::delete_project(borrow_id).unwrap();
    lists()
}

pub fn create(payload: Project) -> Result<bool, bool> {
    dots::project::add_project(&payload)
}
