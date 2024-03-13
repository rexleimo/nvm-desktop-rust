use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub id: Option<i32>,
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
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
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
    let id = match project.id {
        Some(id) => id,
        None => 0,
    };

    let exists = get_project_by_id(&id);

    match exists {
        Ok(_) => {
            update_project(&project);
            Ok(true)
        }
        Err(_) => {
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
                .expect("执行错误");

            match execute {
                key if key > 0 => Ok(true),
                _ => Err(false),
            }
        }
    }
}

pub fn delete_project(id: &u32) -> Result<bool> {
    let conn = open_db();
    let execute = conn.execute("DELETE FROM projects WHERE id = ?1", (&id,))?;
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
                id: row.get(0)?,
                name: row.get(1)?,
                dir: row.get(2)?,
                version: row.get(3)?,
                run_cmd: row.get(4)?,
            })
        })
        .unwrap();
    let mut projects: Vec<Project> = Vec::new();
    for project in projects_iter {
        projects.push(project.unwrap());
    }
    projects
}

fn get_project_by_id(id: &i32) -> Result<Project, rusqlite::Error> {
    let conn = open_db();
    let mut stmt = conn
        .prepare("SELECT * FROM projects WHERE id = ?1")
        .unwrap();
    let project_iter = stmt.query_row([&id], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            dir: row.get(2)?,
            version: row.get(3)?,
            run_cmd: row.get(4)?,
        })
    });
    project_iter
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
                id: row.get(0)?,
                name: row.get(1)?,
                dir: row.get(2)?,
                version: row.get(3)?,
                run_cmd: row.get(4)?,
            })
        })
        .unwrap();
    Some(project_iter)
}

pub fn update_project(project: &Project) -> bool {
    let conn = open_db();
    let sql = "UPDATE projects SET dir = ?1, version = ?2, run_cmd = ?3, name = ?4 WHERE id = ?5";
    let id = match project.id {
        Some(id) => id,
        None => 0,
    };
    let execute = conn.execute(
        sql,
        [
            &project.dir,
            &project.version,
            &project.run_cmd,
            &project.name,
            &id.to_string(),
        ],
    );
    match execute {
        Ok(ex) => ex > 0,
        Err(_) => false,
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_cmd_args(project_name: &str) -> (&str, Vec<&str>) {
    let row = get_project(project_name);
    let directory = row.dir;

    let os_cmd = "bash";
    let args = &["-c", &format!("cd {} && {}", directory, row.run_cmd)];

    (os_cmd, args)
}

#[cfg(target_os = "windows")]
pub fn get_cmd_args(project_name: &String) -> Vec<String> {
    let row = get_project(&project_name).unwrap();
    let directory = row.dir.replace("\\", "/");
    let args = &["/c", &format!("cd /d {} && {}", directory, row.run_cmd)];
    args.iter().map(|s| s.to_string()).collect()
}
