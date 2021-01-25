use chrono::{Date, Utc};
use dirs;
use rusqlite::{params, Connection, Result};
use std::error::Error;

#[derive(Debug)]
struct Chain {
    project_id: i32,
    date: Date<Utc>,
}

#[derive(Debug)]
struct Project {
    id: Option<i32>,
    name: String,
    sunday: bool,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
}

fn add_project(conn: &Connection, project: Project) -> Result<(), Box<dyn Error>> {
    conn.execute(
            "INSERT INTO projects (
                name, 
                sunday, 
                monday, 
                tuesday, 
                wednesday, 
                thursday, 
                friday, 
                saturday
                )
                VALUES (
                    ?1, 
                    ?2, 
                    ?3, 
                    ?4, 
                    ?5, 
                    ?6, 
                    ?7, 
                    ?8
                    )
                ON CONFLICT (name)
                DO UPDATE SET 
                    sunday = ?2,
                    monday = ?3,
                    tuesday = ?4,
                    wednesday = ?5,
                    thursday = ?6,
                    friday = ?7,
                    saturday = ?8
            ;",
            params![project.name,  project.sunday, project.monday, project.tuesday, project.wednesday, project.thursday, project.friday, project.saturday]
            )?;

    Ok(())
}

fn setup_tables(conn: &Connection) -> Result<(), Box<dyn Error>> {
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
                    name            TEXT NOT NULL UNIQUE,
                    sunday          INTEGER NOT NULL,
                    monday          INTEGER NOT NULL,
                    tuesday         INTEGER NOT NULL,
                    wednesday       INTEGER NOT NULL,
                    thursday        INTEGER NOT NULL,
                    friday          INTEGER NOT NULL,
                    saturday        INTEGER NOT NULL
                );",
        params![],
    )?;

    Ok(())

}

fn main() -> Result<(), Box<dyn Error>> {
    let db = dirs::home_dir().unwrap().join(".chain").join("chain_db");
    let conn = Connection::open(db)?;

    let project = Project {
        id: None,
        name: "Project".to_string(),
        sunday: true,
        monday: true,
        tuesday: true,
        wednesday: true,
        thursday: true,
        friday: true,
        saturday: true,
    };

    setup_tables(&conn)?;

    add_project(&conn, project)?;

    Ok(())
}
