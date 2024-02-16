use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "versions";
const DB_URL: &str = "./version.sqlite3";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub id: Option<i64>,
    pub name: String,
    pub status: i16,
    pub is_use: i8,
}

pub fn open_db() -> Connection {
    let conn = Connection::open(&DB_URL).unwrap();
    return conn;
}

pub fn create_or_update_table() {
    let conn = open_db();
    let table_exists: Result<i32, Error> = conn.query_row(
        format!(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='{}'",
            TABLE_NAME
        )
        .as_str(),
        [],
        |row| row.get(0),
    );
    if table_exists == Ok(0) {
        create_table(&conn);
        init_table();
    }
}

fn create_table(conn: &Connection) {
    conn.execute(
        format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id             INTEGER PRIMARY KEY AUTOINCREMENT,
            name           TEXT NOT NULL,
            status         INTEGER NOT NULL,
            is_use         INTEGER
        )",
            TABLE_NAME
        )
        .as_str(),
        [],
    )
    .unwrap();
}

fn init_table() {
    let init_versions = vec![
        "21.6.2", "21.0.0", "20.6.0", "20.11.1", "20.11.0", "20.10.0", "20,9,0", "18.19.1",
        "18.19.0", "18,18,0", "18.12.0",
    ];

    for version_str in init_versions {
        let version = Version {
            id: None,
            name: version_str.to_string(),
            status: 0,
            is_use: 0,
        };
        let _ = insert_version(&version);
    }
}

fn _update_table(_conn: &Connection) {}

pub fn insert_version(version: &Version) -> Result<bool, Error> {
    let conn = open_db();
    let execute = conn.execute(
        format!(
            "INSERT INTO {} (name, status, is_use) VALUES (?1, ?2, ?3)",
            TABLE_NAME
        )
        .as_str(),
        &[
            &version.name,
            &version.status.to_string(),
            &version.is_use.to_string(),
        ],
    );
    println!(
        "{}",
        format!(
            "INSERT INTO {} (name, status, is_use) VALUES (?1, ?2, ?3)",
            TABLE_NAME
        )
        .as_str()
    );
    match execute {
        Ok(size) => {
            return Ok(size > 0);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn get_version(name: &str) -> Result<Version, Error> {
    let conn = open_db();
    let mut stmt =
        conn.prepare(format!("SELECT * FROM {} WHERE name = ?1 limit 1", TABLE_NAME).as_str())?;
    let version = stmt.query_row(&[&name], |row| {
        Ok(Version {
            id: row.get(0)?,
            name: row.get(1)?,
            status: row.get(2)?,
            is_use: row.get(3)?,
        })
    })?;
    Ok(version)
}

pub fn update_version(version: &Version) -> Result<bool, Error> {
    let conn = open_db();
    let execute = conn.execute(
        format!(
            "UPDATE {} SET status = ?1, is_use = ?2 WHERE name = ?3",
            TABLE_NAME
        )
        .as_str(),
        &[
            &version.status.to_string(),
            &version.is_use.to_string(),
            &version.name,
        ],
    );
    match execute {
        Ok(size) => {
            return Ok(size > 0);
        }
        Err(e) => Err(e),
    }
}

pub fn get_all_version() -> Vec<Version> {
    let conn = open_db();
    let sql_str = format!("SELECT * FROM {}", TABLE_NAME);
    println!("{}", sql_str);
    let mut stmt = conn.prepare(sql_str.as_str()).unwrap();
    let version = stmt
        .query_map([], |row| {
            Ok(Version {
                id: row.get(0)?,
                name: row.get(1)?,
                status: row.get(2)?,
                is_use: row.get(3)?,
            })
        })
        .unwrap();
    let mut result: Vec<Version> = Vec::new();
    for v in version {
        let v = v.unwrap();
        result.push(v);
    }
    result
}

pub fn update_version_is_use(update_id: &i64) -> bool {
    let conn = open_db();
    let sql = format!("UPDATE {} SET is_use = ?1 where id = ?2", TABLE_NAME);
    let sql_cur = format!("UPDATE {} SET is_use = ?1 where id <> ?2", TABLE_NAME);
    let id_str = update_id.to_string();
    let set_is_use_false = conn
        .execute(&sql_cur, &[&"0".to_string(), &id_str])
        .unwrap();
    let set_is_use_target = conn.execute(&sql, &[&"1".to_string(), &id_str]).unwrap();
    if set_is_use_false > 0 && set_is_use_target > 0 {
        true
    } else {
        false
    }
}
