use crate::Chain;
use crate::Link;
use anyhow::Result;
use chrono::NaiveDate;
use rusqlite::{params, Connection, NO_PARAMS};

static DATE_FORMAT: &str = "%Y%m%d";

pub fn setup_tables(conn: &Connection) -> Result<()> {
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

pub fn add_chain(conn: &Connection, chain: &Chain) -> Result<()> {
    conn.execute(
        "INSERT INTO chains (name, sunday, monday, tuesday, wednesday, thursday, friday, saturday)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                ON CONFLICT (name)
                DO UPDATE SET sunday = ?2, monday = ?3, tuesday = ?4, wednesday = ?5, thursday = ?6, friday = ?7, saturday = ?8;",
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

pub fn delete_chain_for_name(conn: &Connection, chain_name: &str) -> Result<()> {
    conn.execute("DELETE FROM chains WHERE name=?1", params![chain_name])?;

    Ok(())
}

pub fn edit_chain_for_name(conn: &Connection, chain: &Chain, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE chains 
            SET 
                name = ?2,
                sunday = ?3,
                monday = ?4,
                tuesday = ?5,
                wednesday = ?6,
                thursday = ?7,
                friday = ?8,
                saturday = ?9
            WHERE 
                name = ?1;",
        params![
            name,
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

pub fn get_chains(conn: &Connection) -> Result<Vec<Chain>> {
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
            FROM chains
            ORDER BY name ASC;",
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

pub fn get_chain_id_for_name(conn: &Connection, chain_name: &str) -> Result<i32> {
    Ok(conn.query_row_and_then(
        "SELECT id FROM chains WHERE name=?;",
        params![chain_name],
        |row| row.get(0),
    )?)
}

pub fn get_chain_for_id(conn: &Connection, chain_id: i32) -> Result<Chain> {
    let chain = conn.query_row("SELECT id, name, sunday, monday, tuesday, wednesday, thursday, friday, saturday FROM chains WHERE id=?1;",
            params![chain_id],
            |row| {
            Ok(Chain {
                id: Some(row.get(0)?),
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

    Ok(chain)
}

pub fn get_chain_for_name(conn: &Connection, chain_name: &str) -> Result<Chain> {
    let chain = conn.query_row("SELECT id, name, sunday, monday, tuesday, wednesday, thursday, friday, saturday FROM chains WHERE name=?1;",
            params![chain_name],
            |row| {
            Ok(Chain {
                id: Some(row.get(0)?),
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

    Ok(chain)
}

pub fn add_link(conn: &Connection, link: &Link) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO links (chain_id, date)
                VALUES (?1, ?2);",
        params![link.chain_id, link.date.format(DATE_FORMAT).to_string()],
    )?;

    Ok(())
}

pub fn delete_link(conn: &Connection, link: &Link) -> Result<()> {
    conn.execute(
        "DELETE FROM links WHERE chain_id=?1 AND date=?2;",
        params![link.chain_id, link.date.format("%Y%m%d").to_string()],
    )?;

    Ok(())
}

pub fn get_links_for_chain_id(conn: &Connection, chain_id: i32) -> Result<Vec<Link>> {
    let mut statement = conn.prepare(
        "SELECT chain_id, date 
            FROM links 
            WHERE chain_id = ? 
            ORDER BY date ASC;",
    )?;

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
