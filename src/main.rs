use anyhow::{anyhow, Result};
use chain::database;
use chain::logic;
use chain::printer;
use chain::{Chain, Link, Streak};
use chrono::{NaiveDate, Local};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use dirs;
use rusqlite::Connection;

// Cargo Information
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Link Manipulation Commands
const ADD: &'static str = "add";
const MV: &'static str = "mv";
const RM: &'static str = "rm";

// Chain Manipulation Commands
const ADD_CHAIN: &'static str = "add-chain";
const RENAME_CHAIN: &'static str = "rename-chain";
const RM_CHAIN: &'static str = "rm-chain";

// Chain Information Commands
const DUE: &'static str = "due";
const LS: &'static str = "ls";
const STATUS: &'static str = "status";

// Argument Names
const CHAIN: &'static str = "CHAIN";
const MACHINE: &'static str = "machine";
const CURRENT: &'static str = "CURRENT";
const NEW: &'static str = "NEW";
const DATE: &'static str = "DATE";

const FORMAT: &'static str = "%Y-%m-%d";

fn add(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let name = m.value_of(CHAIN).unwrap();

    let date = if m.is_present(DATE) {
        NaiveDate::parse_from_str(m.value_of(DATE).unwrap(), FORMAT)?
    } else {
        Local::today().naive_local()
    };

    let id = database::get_chain_id_for_name(&conn, &name)?;
    let chain = database::get_chain_for_id(&conn, id)?;

    let link = Link { chain_id: id, date };

    database::add_link(&conn, &link)?;

    let links = database::get_links_for_chain_id(&conn, id)?;
    let streak = logic::calculate_streak(&chain, &links);

    printer::print_add(&chain, &link);
    printer::print_streak(&streak);

    Ok(())
}

fn mv(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let name = m.value_of(CHAIN).unwrap();

    let current_date = NaiveDate::parse_from_str(m.value_of(CURRENT).unwrap(), FORMAT)?;
    let new_date = NaiveDate::parse_from_str(m.value_of(NEW).unwrap(), FORMAT)?;

    let id = database::get_chain_id_for_name(&conn, &name)?;
    let chain = database::get_chain_for_name(&conn, &name)?;

    let current = Link { chain_id: id, date: current_date };
    let new = Link { chain_id: id, date: new_date };

    database::update(conn, &current, &new)?;
    printer::print_mv(&chain, &current, &new);

    Ok(())
}

fn rm(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let date = NaiveDate::parse_from_str(m.value_of(DATE).unwrap(), FORMAT)?;
    let name = m.value_of(CHAIN).unwrap();

    let id = database::get_chain_id_for_name(&conn, &name)?;
    let chain = database::get_chain_for_name(&conn, &name)?;

    let link = Link { chain_id: id, date };

    database::delete_link(&conn, &link)?;
    printer::print_rm(&chain, &link);

    Ok(())
}

fn add_chain(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let name = m.value_of(CHAIN).unwrap();

    let chain = Chain {
        id: -1,
        name: name.to_string(),
    };

    database::add_chain(&conn, &chain)?;
    printer::print_add_chain(&chain);

    Ok(())
}

fn rename_chain(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let current = m.value_of(CURRENT).unwrap();
    let new = m.value_of(NEW).unwrap();

    let chain = Chain {
       id: -1,
       name: current.to_string(),
    };

    database::edit_chain_for_name(&conn, &chain, &new)?;
    printer::print_rename_chain(&chain, &new);

    Ok(())
}

fn rm_chain(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let name = m.value_of(CHAIN).unwrap();

    let id = database::get_chain_id_for_name(&conn, &name)?;
    let chain = database::get_chain_for_name(&conn, &name)?;

    let links = database::get_links_for_chain_id(&conn, id)?;

    for link in links.iter() {
        database::delete_link(&conn, &link)?;
    }

    database::delete_chain_for_name(&conn, &name)?;
    printer::print_rm_chain(&chain);

    Ok(())
}


fn due(conn: &Connection, m: &ArgMatches) -> Result<()> {
    let chains = database::get_chains(&conn)?;

    let mut due: Vec<Streak> = Vec::new();

    for chain in chains.iter() {
        let id = chain.id;
        let links = database::get_links_for_chain_id(&conn, id as i32)?;

        let latest = links.last();
        let today = Local::today().naive_local();

        if latest.is_some() {
            // Check if the chain has a link for today. 
            // signed_duration_since will be zero if there is a link for today.
            if today.signed_duration_since(latest.unwrap().date).num_days() > 0 {   
                let streak = logic::calculate_streak(&chain, &links);
                due.push(streak);
            }
        } else {
            let streak = Streak {
                name: chain.name.to_string(),
                streak: 0,
                longest_streak: 0,
            };

            due.push(streak);
        }
    }

    if m.is_present(MACHINE) {
        printer::print_streaks_machine(&due); 
    } else {
        printer::print_streaks(&due);
    }

    Ok(())
}

fn ls(conn: &Connection, _m: &ArgMatches) -> Result<()> {
    let chains = database::get_chains(&conn)?;

    printer::print_ls(&chains);

    Ok(())
}

fn status(conn: &Connection, m: &ArgMatches) -> Result<()> {
    // print the status of a single chain if the name of the chain is provided.
    if m.is_present(CHAIN) {
        let name = m.value_of(CHAIN).unwrap();

        let id = database::get_chain_id_for_name(&conn, &name)?;
        let chain = database::get_chain_for_id(&conn, id)?;
        let links = database::get_links_for_chain_id(&conn, id)?;

        let streak = logic::calculate_streak(&chain, &links);

        printer::print_streak(&streak);
    } else {
        let chains = database::get_chains(&conn)?;

        let mut streaks: Vec<Streak> = Vec::new();

        for chain in chains.iter() {
            let chain_id = chain.id;
            let links = database::get_links_for_chain_id(&conn, chain_id as i32)?;
            let streak = logic::calculate_streak(&chain, &links);

            streaks.push(streak);
        }

        printer::print_streaks(&streaks);
    }

    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new(NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommand(
            SubCommand::with_name(ADD)
                .arg(
                    Arg::with_name("CHAIN")
                        .required(true)
                        .index(1)
                        .help("the name of the chain")
                )
                .arg(
                    Arg::with_name("DATE")
                        .required(false)
                        .index(2)
                        .help("the date to add")
                )
        )
        .subcommand(
            SubCommand::with_name(MV)
                .arg(
                    Arg::with_name(CHAIN)
                        .required(true)
                        .index(1)
                        .help("the name of the chain")
                    )
                .arg(Arg::with_name(CURRENT)
                    .required(true)
                    .index(2)
                    .help("the current date")
                    )
                .arg(Arg::with_name(NEW)
                    .required(true)
                    .index(3)
                    .help("the new date")
                    )
                )
        .subcommand(
            SubCommand::with_name(RM)
                .arg(
                    Arg::with_name("CHAIN")
                        .required(true)
                        .index(1)
                        .help("the name of the chain")
                )
                .arg(
                    Arg::with_name("DATE")
                        .required(true)
                        .index(2)
                        .help("the date to delete")
                )
        )
        .subcommand(
            SubCommand::with_name(ADD_CHAIN).arg(
                Arg::with_name("CHAIN")
                    .index(1)
                    .required(true)
                    .help("the name of the chain")
            )
        )
        .subcommand(
            SubCommand::with_name(RENAME_CHAIN)
                .arg(
                    Arg::with_name(CURRENT)
                        .required(true)
                        .index(1)
                        .help("the current name of the chain")
                )
                .arg(
                    Arg::with_name(NEW)
                        .required(true)
                        .index(2)
                        .help("the new name of the chain")
                )
        )
        .subcommand(
            SubCommand::with_name(RM_CHAIN).arg(
                Arg::with_name("CHAIN")
                    .required(true)
                    .index(1)
                    .help("the name of the chain")
            )
        )
        .subcommand(
            SubCommand::with_name(DUE).arg(
                Arg::with_name("machine")
                    .long("machine")
                    .short("m")
                    .required(false)
                    .help("provide output in a machine readable format")
            )
        )
        .subcommand(SubCommand::with_name(LS))
        .subcommand(
            SubCommand::with_name(STATUS).arg(
                Arg::with_name("CHAIN")
                    .required(false)
                    .index(1)
                    .help("the name of the chain")
            )
        )
        .get_matches();


    // Setup the database
    let db = dirs::home_dir()
        .ok_or(anyhow!("Failed to locate the users home directory"))?
        .join(".c")
        .join("c.db");
    let conn = Connection::open(db)?;

    database::setup_tables(&conn)?;

    // Run subcommand
    match matches.subcommand() {
        (ADD, Some(m)) => add(&conn, m)?,
        (MV, Some(m)) => mv(&conn, m)?,
        (RM, Some(m)) => rm(&conn, m)?,
        (ADD_CHAIN, Some(m)) => add_chain(&conn, m)?,
        (RENAME_CHAIN, Some(m)) => rename_chain(&conn, m)?,
        (RM_CHAIN, Some(m)) => rm_chain(&conn, m)?,
        (DUE, Some(m)) => due(&conn, m)?,
        (LS, Some(m)) => ls(&conn, m)?,
        (STATUS, Some(m)) => status(&conn, m)?,
        _ => return Err(anyhow!("Failed to parse subcommand")),
    };


    Ok(())
}
