use chrono::{NaiveDate, Utc};
use dirs;
use rusqlite::{params, Connection, Result};
use std::error::Error;

#[derive(Debug)]
struct Link {
    chain_id: i32,
    date: NaiveDate,
}

#[derive(Debug)]
struct Chain {
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

fn setup_tables(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chains (
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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS links (
                    chain_id        INTEGER,
                    date            TEXT NOT NULL,
                    PRIMARY KEY (chain_id, date),
                    FOREIGN KEY (chain_id) REFERENCES chains(id)
                );",
        params![],
    )?;

    Ok(())
}

fn add_chain(conn: &Connection, chain: &Chain) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO chains (
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
        params![
            chain.name,
            chain.sunday,
            chain.monday,
            chain.tuesday,
            chain.wednesday,
            chain.thursday,
            chain.friday,
            chain.saturday
        ],
    )?;

    Ok(())
}

fn get_chain_id(conn: &Connection, chain_name: &str) -> Result<i32, Box<dyn Error>> {
    Ok(conn.query_row_and_then(
        "SELECT id FROM chains WHERE name=?;",
        params![chain_name],
        |row| row.get(0),
    )?)
}

fn add_link(conn: &Connection, link: &Link) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT OR IGNORE INTO links (
                chain_id,
                date
                )
                VALUES (
                    ?1,
                    ?2
            );",
        params![link.chain_id, link.date.format("%Y%m%d").to_string()],
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let db = dirs::home_dir().unwrap().join(".chain").join("chain_db");
    let conn = Connection::open(db)?;

    let chain = Chain {
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
    add_chain(&conn, &chain)?;

    let chain_name = &chain.name;
    let chain_id = get_chain_id(&conn, &chain_name)?;

    println!("chain_id = {}", chain_id);

    let link_one = Link {
        chain_id: chain_id,
        date: Utc::today().naive_utc().pred(),
    };

    let link_two = Link {
        chain_id: chain_id,
        date: Utc::today().naive_utc(),
    };

    let link_three = Link {
        chain_id: chain_id,
        date: Utc::today().naive_utc().succ(),
    };

    add_link(&conn, &link_one)?;
    add_link(&conn, &link_two)?;
    add_link(&conn, &link_three)?;

    let mut statement = conn.prepare("SELECT chain_id, date FROM links WHERE chain_id = ?;")?;
    let link_iter = statement.query_map(params![chain_id], |row| {
        let chain_id: i32 = row.get(0)?;
        let date_str: String = row.get::<usize, String>(1)?.to_string();
        let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").unwrap();

        Ok(Link {
            chain_id: chain_id,
            date: date,
        })
    })?;

    for link in link_iter {
        println!("Found link {:?}", link.unwrap());
    }

    Ok(())
}
