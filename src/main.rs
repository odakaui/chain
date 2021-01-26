use anyhow::Result;
use chain::Chain;
use chain_error::ChainError;
use chrono::{Datelike, Utc, Weekday};
use database::{
    add_chain, add_link, delete_chain_for_name, delete_link, get_chain_id_for_name, get_chain_for_id, get_chains,
    get_links_for_chain_id, setup_tables,
};
use dirs;
use link::Link;
use rusqlite::{params, Connection};

mod chain;
mod chain_error;
mod database;
mod link;

fn main() -> Result<()> {
    let db = dirs::home_dir()
        .ok_or(ChainError::new("Failed to locate home directory"))?
        .join(".chain")
        .join("chain_db");
    let conn = Connection::open(db)?;

    setup_tables(&conn)?;

    for x in 0..10 {
        let chain = Chain {
            id: None,
            name: format!("Chain {}", x + 1).to_string(),
            sunday: rand::random(),
            monday: rand::random(),
            tuesday: rand::random(),
            wednesday: rand::random(),
            thursday: rand::random(),
            friday: rand::random(),
            saturday: rand::random(),
        };

        add_chain(&conn, &chain)?;
    }

    let chains = get_chains(&conn)?;

    for chain in chains.iter() {
        println!("Found {:?}", chain);
    }

    // Add link for chain
    let chain_name = "Chain 1";
    let chain_id = get_chain_id_for_name(&conn, chain_name)?;

    let chain = get_chain_for_id(&conn, chain_id)?;

    let mut date = Utc::today().naive_utc();
    let mut i = 0;

    while i < 100 {
        let weekday = date.weekday();

        println!("{:?}", weekday);

        let is_valid = match weekday {
            Weekday::Sun => chain.sunday,
            Weekday::Mon => chain.monday,
            Weekday::Tue => chain.tuesday,
            Weekday::Wed => chain.wednesday,
            Weekday::Thu => chain.thursday,
            Weekday::Fri => chain.friday,
            Weekday::Sat => chain.saturday,
        };

        if is_valid {
            let link = Link { chain_id, date };

            add_link(&conn, &link)?;

            i += 1;
        } else {
            println!("Invalid date");
        }

        date = date.succ();
    }
    let links = get_links_for_chain_id(&conn, chain_id)?;

    for (i, link) in links.iter().enumerate() {
        println!("{}. Found {:?}", i + 1, link);
    }

    //    let chain_name = &chain_two.name;
    //
    //    delete_chain_for_name(&conn, chain_name)?;
    //
    //    let chains = get_chains(&conn)?;
    //
    //    for chain in chains.iter() {
    //        println!("Found {:?}", chain);
    //    }
    //
    //    let chain_name = &chain_one.name;
    //    let chain_id = get_chain_id(&conn, &chain_name)?;
    //
    //    println!("chain_id = {}", chain_id);
    //
    //    let link_one = Link {
    //        chain_id: chain_id,
    //        date: Utc::today().naive_utc().pred(),
    //    };
    //
    //    let link_two = Link {
    //        chain_id: chain_id,
    //        date: Utc::today().naive_utc(),
    //    };
    //
    //    let link_three = Link {
    //        chain_id: chain_id,
    //        date: Utc::today().naive_utc().succ(),
    //    };
    //
    //    add_link(&conn, &link_one)?;
    //    add_link(&conn, &link_two)?;
    //    add_link(&conn, &link_three)?;
    //
    //
    //    let link = &link_two;
    //
    //    delete_link(&conn, link)?;
    //
    //    let links = get_links_for_chain_id(&conn, chain_id)?;
    //
    //    for link in links.iter() {
    //        println!("Found {:?}", link);
    //    }

    Ok(())
}
