use anyhow::Result;
use chrono::{NaiveDate, Utc};
use dirs;
use rusqlite::{params, Connection, NO_PARAMS};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ChainError {
    details: String,
}

impl ChainError {
    fn new(msg: &str) -> ChainError {
        ChainError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ChainError {
    fn description(&self) -> &str {
        &self.details
    }
}

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

fn setup_tables(conn: &Connection) -> Result<()> {
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

fn add_chain(conn: &Connection, chain: &Chain) -> Result<()> {
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

fn delete_chain_for_name(conn: &Connection, chain_name: &str) -> Result<()> {
    let chain_id = get_chain_id(&conn, chain_name)?;
    conn.execute("DELETE FROM chains WHERE id=?", params![chain_id])?;

    Ok(())
}

fn get_chains(conn: &Connection) -> Result<Vec<Chain>> {
    let mut statement = conn.prepare(
        "SELECT 
                id, 
                name, 
                sunday, 
                monday, 
                tuesday, 
                wednesday, 
                thursday, 
                friday, 
                saturday 
            FROM chains;",
    )?;
    let chain_iter = statement.query_map(NO_PARAMS, |row| {
        Ok(Chain {
            id: row.get(0)?,
            name: row.get(1)?,
            sunday: row.get(2)?,
            monday: row.get(3)?,
            tuesday: row.get(4)?,
            wednesday: row.get(5)?,
            thursday: row.get(6)?,
            friday: row.get(7)?,
            saturday: row.get(8)?,
        })
    })?;

    Ok(chain_iter.filter_map(Result::ok).collect())
}

fn get_chain_id(conn: &Connection, chain_name: &str) -> Result<i32> {
    Ok(conn.query_row_and_then(
        "SELECT id FROM chains WHERE name=?;",
        params![chain_name],
        |row| row.get(0),
    )?)
}

fn add_link(conn: &Connection, link: &Link) -> Result<()> {
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

fn get_links_for_chain_id(conn: &Connection, chain_id: i32) -> Result<Vec<Link>> {
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

    Ok(link_iter.filter_map(Result::ok).collect())
}

fn main() -> Result<()> {
    let db = dirs::home_dir()
        .ok_or(ChainError::new("Failed to locate home directory"))?
        .join(".chain")
        .join("chain_db");
    let conn = Connection::open(db)?;

    let chain_one = Chain {
        id: None,
        name: "Chain One".to_string(),
        sunday: true,
        monday: true,
        tuesday: true,
        wednesday: true,
        thursday: true,
        friday: true,
        saturday: true,
    };

    let chain_two = Chain {
        id: None,
        name: "Chain Two".to_string(),
        sunday: true,
        monday: true,
        tuesday: true,
        wednesday: true,
        thursday: true,
        friday: true,
        saturday: true,
    };

    let chain_three = Chain {
        id: None,
        name: "Chain Three".to_string(),
        sunday: true,
        monday: true,
        tuesday: true,
        wednesday: true,
        thursday: true,
        friday: true,
        saturday: true,
    };

    setup_tables(&conn)?;
    add_chain(&conn, &chain_one)?;
    add_chain(&conn, &chain_two)?;
    add_chain(&conn, &chain_three)?;

    let chains = get_chains(&conn)?;

    for chain in chains.iter() {
        println!("Found {:?}", chain);
    }

    let chain_name = &chain_two.name;

    delete_chain_for_name(&conn, chain_name)?;

    let chains = get_chains(&conn)?;

    for chain in chains.iter() {
        println!("Found {:?}", chain);
    }

    let chain_name = &chain_one.name;
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

    let links = get_links_for_chain_id(&conn, chain_id)?;

    for link in links.iter() {
        println!("Found {:?}", link);
    }

    Ok(())
}
