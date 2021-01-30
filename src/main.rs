use anyhow::{bail, Result};
use chain::database;
use chain::logic;
use chain::printer;
use chain::Chain;
use chain::ChainError;
use chain::Link;
use chrono::{Datelike, Local, NaiveDate};
use clap::{App, Arg, SubCommand};
use dirs;
use rusqlite::Connection;

static WEEKENDS: &str = "weekends";
static WEEKDAYS: &str = "weekdays";
static CUSTOM: &str = "custom";
static ALL: &str = "all";

fn main() -> Result<()> {
    let matches = App::new("Chain")
        .version("0.1.0")
        .author("Odaka Ui <31593320+odakaui@users.noreply.github.com>")
        .about("A simple habit tracking app.")
        .subcommand(
            SubCommand::with_name("streak").arg(
                Arg::with_name("chain")
                    .value_name("CHAIN")
                    .required(false)
                    .index(1)
                    .takes_value(true)
                    .help("The chain's name"),
            ),
        )
        .subcommand(
            SubCommand::with_name("today").arg(
                Arg::with_name("number")
                    .long("number")
                    .short("n")
                    .required(false)
                    .help("Print output as a number instead of list"),
            ),
        )
        .subcommand(
            SubCommand::with_name("add-chain")
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .index(1)
                        .required(true)
                        .takes_value(true)
                        .help("The chain's name"),
                )
                .arg(
                    Arg::with_name(ALL)
                        .long(ALL)
                        .takes_value(false)
                        .requires("chain")
                        .conflicts_with_all(&["weekend, weekends", CUSTOM]),
                )
                .arg(
                    Arg::with_name(WEEKDAYS)
                        .long(WEEKDAYS)
                        .takes_value(false)
                        .requires("chain"),
                )
                .arg(
                    Arg::with_name(WEEKENDS)
                        .long(WEEKENDS)
                        .takes_value(false)
                        .requires("chain"),
                )
                .arg(
                    Arg::with_name(CUSTOM)
                        .long("FILTER")
                        .takes_value(true)
                        .requires("chain"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete-chain").arg(
                Arg::with_name("chain")
                    .value_name("CHAIN")
                    .required(true)
                    .index(1)
                    .takes_value(true)
                    .help("The chain's name"),
            ),
        )
        .subcommand(
            SubCommand::with_name("edit-chain")
                .arg(
                    Arg::with_name("target")
                        .value_name("TARGET")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .help("The chain's current name"),
                )
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .required(true)
                        .index(2)
                        .takes_value(true)
                        .help("The chain's new name"),
                )
                .arg(
                    Arg::with_name(ALL)
                        .long(ALL)
                        .takes_value(false)
                        .requires("chain")
                        .conflicts_with_all(&["weekend, weekends", CUSTOM]),
                )
                .arg(
                    Arg::with_name(WEEKDAYS)
                        .long(WEEKDAYS)
                        .takes_value(false)
                        .requires("chain"),
                )
                .arg(
                    Arg::with_name(WEEKENDS)
                        .long(WEEKENDS)
                        .takes_value(false)
                        .requires("chain"),
                )
                .arg(
                    Arg::with_name(CUSTOM)
                        .long("FILTER")
                        .takes_value(true)
                        .requires("chain"),
                ),
        )
        .subcommand(SubCommand::with_name("list-chains"))
        .subcommand(
            SubCommand::with_name("add-link")
                .arg(
                    Arg::with_name("date")
                        .value_name("DATE")
                        .index(2)
                        .required(false)
                        .takes_value(true)
                        .help("Link date"),
                )
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .help("The chain's name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete-link")
                .arg(
                    Arg::with_name("date")
                        .value_name("DATE")
                        .index(2)
                        .takes_value(true)
                        .required(true)
                        .help("Link date"),
                )
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .help("The chain's name"),
                ),
        )
        .get_matches();

    let db = dirs::home_dir()
        .ok_or(ChainError::new("Failed to locate home directory"))?
        .join(".chain")
        .join("chain_db");
    let conn = Connection::open(db)?;

    database::setup_tables(&conn)?;

    if let Some(matches) = matches.subcommand_matches("streak") {
        if matches.is_present("chain") {
            let chain_name = matches.value_of("chain").unwrap();
            let chain_id = database::get_chain_id_for_name(&conn, &chain_name)?;
            let chain = database::get_chain_for_id(&conn, chain_id)?;
            let links = database::get_links_for_chain_id(&conn, chain_id)?;
            let streak = logic::calculate_streak(&chain, &links);

            printer::print_streak(&streak);
        } else {
            let chains = database::get_chains(&conn)?;

            for chain in chains.iter() {
                let chain_id = chain.id.unwrap();
                let links = database::get_links_for_chain_id(&conn, chain_id)?;
                let streak = logic::calculate_streak(&chain, &links);

                printer::print_chain_name(&chain);
                printer::print_streak(&streak);
                println!("");
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("today") {
        // TODO cleanup "today"
        let chains = database::get_chains(&conn)?;
        let mut number = 0;
        let mut total = 0;

        for chain in chains.iter() {
            let chain_id = chain.id.unwrap();
            let links = database::get_links_for_chain_id(&conn, chain_id)?;
            let latest_link = links.last();

            let today = Local::today().naive_local();
            if latest_link.is_some()
                && logic::is_valid(&chain, &today.weekday())
                && today
                    .signed_duration_since(latest_link.unwrap().date)
                    .num_days()
                    > 0
            {
                if matches.is_present("number") {
                    number += 1;
                } else {
                    let streak = logic::calculate_streak(&chain, &links);

                    printer::print_chain_name(&chain);
                    printer::print_streak(&streak);
                    println!("");
                }

                total += 1;
            } else if latest_link.is_none() && logic::is_valid(&chain, &today.weekday()) {
                if matches.is_present("number") {
                    number += 1;
                } else {
                    printer::print_chain_name(&chain);
                    println!("Current streak: 0");
                    println!("Longest streak: 0");
                    println!("");
                }

                total += 1;
            }
        }

        if matches.is_present("number") {
            println!("{}", number);
        } else if total == 0 {
            println!("Congratulations, you have completed all of your chains for today");
        }
    } else if let Some(matches) = matches.subcommand_matches("add-chain") {
        let chain_name = matches.value_of("chain").unwrap();

        let chain: Chain;
        let filter: &str;

        if matches.is_present(WEEKDAYS) {
            filter = WEEKDAYS;
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: false,
                monday: true,
                tuesday: true,
                wednesday: true,
                thursday: true,
                friday: true,
                saturday: false,
            };
        } else if matches.is_present(WEEKENDS) {
            filter = WEEKENDS;
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: true,
                monday: false,
                tuesday: false,
                wednesday: false,
                thursday: false,
                friday: false,
                saturday: true,
            };
        } else if matches.is_present(CUSTOM) {
            todo!()
        } else {
            filter = ALL;
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: true,
                monday: true,
                tuesday: true,
                wednesday: true,
                thursday: true,
                friday: true,
                saturday: true,
            };
        }

        database::add_chain(&conn, &chain)?;

        println!("Added \"{}\" with a filter of \"{}\"", &chain.name, filter);
    } else if let Some(matches) = matches.subcommand_matches("delete-chain") {
        let chain_name = matches.value_of("chain").unwrap();
        let chain_id = database::get_chain_id_for_name(&conn, &chain_name)?;
        let links = database::get_links_for_chain_id(&conn, chain_id)?;

        for link in links.iter() {
            database::delete_link(&conn, &link)?;
        }

        database::delete_chain_for_name(&conn, &chain_name)?;

        println!("Deleted \"{}\"", &chain_name);
    } else if let Some(matches) = matches.subcommand_matches("edit-chain") {
        let target_name = matches.value_of("target").unwrap();
        let chain_name = matches.value_of("chain").unwrap();

        let chain: Chain;

        if matches.is_present(WEEKDAYS) {
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: false,
                monday: true,
                tuesday: true,
                wednesday: true,
                thursday: true,
                friday: true,
                saturday: false,
            };
        } else if matches.is_present(WEEKENDS) {
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: true,
                monday: false,
                tuesday: false,
                wednesday: false,
                thursday: false,
                friday: false,
                saturday: true,
            };
        } else if matches.is_present(CUSTOM) {
            todo!()
        } else {
            chain = Chain {
                id: None,
                name: chain_name.to_string(),
                sunday: true,
                monday: true,
                tuesday: true,
                wednesday: true,
                thursday: true,
                friday: true,
                saturday: true,
            };
        }

        database::edit_chain_for_name(&conn, &chain, &target_name)?;

        println!("Updated \"{}\"", &chain.name);
    } else if let Some(_) = matches.subcommand_matches("list-chains") {
        let chains = database::get_chains(&conn)?;

        for chain in chains.iter() {
            println!("{}", chain.name);
        }
    } else if let Some(matches) = matches.subcommand_matches("add-link") {
        let chain_name = matches.value_of("chain").unwrap();
        let date: NaiveDate;

        if matches.is_present("date") {
            date = NaiveDate::parse_from_str(matches.value_of("date").unwrap(), "%Y-%m-%d")?;
        } else {
            date = Local::today().naive_local();
        }

        let chain_id = match database::get_chain_id_for_name(&conn, &chain_name) {
            Ok(id) => id,
            Err(e) => return Err(e), // TODO: Make error handling more robust, or at least clean up the message
        };

        let link = Link { chain_id, date };
        let chain = database::get_chain_for_id(&conn, chain_id)?;

        if !logic::check_link(&chain, &link) {
            bail!(
                "Failed to add the link for \"{}\" to \"{}\", because the filter does not allow links to be created on \"{}\"",
                &date.format("%Y-%m-%d"),
                &chain.name,
                &date.weekday()
            );
        }

        database::add_link(&conn, &link)?;

        let links = database::get_links_for_chain_id(&conn, chain_id)?;
        let streak = logic::calculate_streak(&chain, &links);

        println!(
            "Added link for \"{}\" to \"{}\"",
            date.format("%Y-%m-%d"),
            chain_name
        );
        printer::print_streak(&streak);
    } else if let Some(matches) = matches.subcommand_matches("delete-link") {
        let date = NaiveDate::parse_from_str(matches.value_of("date").unwrap(), "%Y-%m-%d")?;
        let chain_name = matches.value_of("chain").unwrap();
        let chain_id = database::get_chain_id_for_name(&conn, &chain_name)?;

        let link = Link { chain_id, date };

        database::delete_link(&conn, &link)?;

        println!(
            "Deleted link for \"{}\" from \"{}\"",
            date.format("%Y-%m-%d"),
            chain_name
        );
    }

    //
    //    setup_tables(&conn)?;
    //
    //    for x in 0..10 {
    //        let chain = Chain {
    //            id: None,
    //            name: format!("Chain {}", x + 1).to_string(),
    //            sunday: rand::random(),
    //            monday: rand::random(),
    //            tuesday: rand::random(),
    //            wednesday: rand::random(),
    //            thursday: rand::random(),
    //            friday: rand::random(),
    //            saturday: rand::random(),
    //        };
    //
    //        add_chain(&conn, &chain)?;
    //    }
    //
    //    let chains = get_chains(&conn)?;
    //
    //    for chain in chains.iter() {
    //        println!("Found {:?}", chain);
    //    }
    //
    //    let chain_name = "Chain 1";
    //    let chain_id = get_chain_id_for_name(&conn, chain_name)?;
    //
    //    let chain = get_chain_for_id(&conn, chain_id)?;
    //
    //    let mut date = Local::today().naive_utc();
    //    let mut i = 0;
    //
    //    while i < 100 {
    //        let link = Link {
    //            chain_id: chain_id,
    //            date: date,
    //        };
    //
    //        if logic::check_link(&chain, &link) {
    //            database::add_link(&conn, &link)?;
    //
    //            i += 1
    //        }
    //
    //        date = date.succ();
    //    }
    //
    //    let links = get_links_for_chain_id(&conn, chain_id)?;
    //
    //    for (i, link) in links.iter().enumerate() {
    //        if i == 9 {
    //            database::delete_link(&conn, &link)?;
    //        }
    //    }
    //
    //    let links = get_links_for_chain_id(&conn, chain_id)?;
    //
    //    for (i, link) in links.iter().enumerate() {
    //        println!("{}. Found {:?}", i + 1, link);
    //    }
    //
    //    let streak = logic::calculate_streak(&chain, &links);
    //
    //    println!("Longest streak: {}", streak.longest_streak);
    //    println!("Current streak: {}", streak.streak);
    //
    //
    Ok(())
}
