use chrono::{Date, Utc};
use dirs;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Chain {
    project_id: i32,
    date: Date<Utc>,
}

#[derive(Debug)]
struct Project {
    id: i32,
    name: String,
    filter: u8,
}

fn main() -> Result<()> {
    let db = dirs::home_dir().unwrap().join(".chain").join("chain_db");
    let conn = Connection::open(db)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS chains (
                    project_id      INTEGER,
                    date            DATE NOT NULL,
                    PRIMARY KEY (project_id, date),
                    FOREIGN KEY (project_id) REFERENCES projects(id)
                );",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
                    id              INTEGER PRIMARY KEY,
                    name            TEXT NOT NULL,
                    filter          INTEGER
                );",
        params![],
    )?;

    Ok(())
}
