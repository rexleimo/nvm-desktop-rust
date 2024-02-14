use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub name: String,
    pub dir: String,
    pub version: String,
    pub run_cmd: String,
}

pub fn open_db() -> Connection {
    let conn = Connection::open("./project.sqlite3").unwrap();
    conn
}

pub fn create_project() -> Result<()> {
    let conn = open_db();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            name     TEXT NOT NULL,
            dir      TEXT NOT NULL,
            version  TEXT NOT NULL,
            run_cmd  TEXT NOT NULL
      )",
        (),
    )?;
    Ok(())
}

pub fn add_project(project: &Project) -> Result<bool, bool> {
    let conn = open_db();
    let execute = conn
        .execute(
            "INSERT INTO projects (name, dir, version, run_cmd) VALUES (?1, ?2, ?3, ?4)",
            (
                &project.name,
                &project.dir,
                &project.version,
                &project.run_cmd,
            ),
        )
        .unwrap();

    match execute {
        key if key > 0 => Ok(true),
        _ => Err(false),
    }
}

pub fn delete_project(project_name: &String) -> Result<bool> {
    let conn = open_db();
    let execute = conn.execute("DELETE FROM projects WHERE name = ?1", (&project_name,))?;
    match execute {
        key if key > 0 => Ok(true),
        _ => Ok(false),
    }
}

pub fn get_projects() -> Vec<Project> {
    let conn = open_db();
    let mut stmt = conn.prepare("SELECT * FROM projects").unwrap();
    let projects_iter = stmt
        .query_map([], |row| {
            Ok(Project {
                name: row.get(0)?,
                dir: row.get(1)?,
                version: row.get(2)?,
                run_cmd: row.get(3)?,
            })
        })
        .unwrap();
    let mut projects: Vec<Project> = Vec::new();
    for project in projects_iter {
        projects.push(project.unwrap());
    }
    projects
}

pub fn get_project(project_name: &String) -> Option<Project> {
    // 获取一个project对象
    let conn = open_db();
    let mut stmt = conn
        .prepare("SELECT * FROM projects WHERE name = ?1")
        .unwrap();
    let project_iter = stmt
        .query_row([&project_name], |row| {
            Ok(Project {
                name: row.get(0)?,
                dir: row.get(1)?,
                version: row.get(2)?,
                run_cmd: row.get(3)?,
            })
        })
        .unwrap();
    Some(project_iter)
}

