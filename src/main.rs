use anyhow::{bail, Result};
use chain::database;
use chain::logic;
use chain::Chain;
use chain::ChainError;
use chain::Link;
use chrono::{NaiveDate, Utc};
use clap::{App, Arg, SubCommand};
use dirs;
use rusqlite::{Connection};

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
                    .help("Name of chain to add link to"),
            ),
        )
        // TODO edit or update a LINK
        // TODO edit or update a CHAIN
        .subcommand(
            SubCommand::with_name("add-link")
                .arg(
                    Arg::with_name("date")
                        .value_name("DATE")
                        .index(2)
                        .required(false)
                        .takes_value(true)
                        .help("Create link on date."),
                )
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .help("Name of chain to add link to"),
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
                        .help("Create link on date."),
                )
                .arg(
                    Arg::with_name("chain")
                        .value_name("CHAIN")
                        .required(true)
                        .index(1)
                        .takes_value(true)
                        .help("Name of chain to add link to"),
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
                        .help("Name of chain"),
                )
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .takes_value(false)
                        .requires("chain")
                        .conflicts_with_all(&["weekend, weekend", "custom"])
                        .help("Allow link creation on weekdays only."),
                )
                .arg(
                    Arg::with_name("weekday")
                        .long("weekday")
                        .takes_value(false)
                        .requires("chain")
                        .help("Allow link creation on weekdays only."),
                )
                .arg(
                    Arg::with_name("weekend")
                        .long("weekend")
                        .takes_value(false)
                        .requires("chain")
                        .help("Allow link creation on weekends only."),
                )
                .arg(
                    Arg::with_name("custom")
                        .long("FILTER")
                        .takes_value(true)
                        .requires("chain")
                        .help("Create a custom filter."),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete-chain").arg(
                Arg::with_name("chain")
                    .value_name("CHAIN")
                    .required(true)
                    .index(1)
                    .takes_value(true)
                    .help("Name of chain to add link to"),
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

            println!("{:?}", links);
            let streak = logic::calculate_streak(&chain, &links);

            println!("Current streak: {}", streak.streak);
            println!("Longest streak: {}", streak.longest_streak);
        } else {
            let chains = database::get_chains(&conn)?;

            for chain in chains.iter() {
                let chain_id = chain.id.unwrap();
                let links = database::get_links_for_chain_id(&conn, chain_id)?;

                println!("{:?}", links);

                let streak = logic::calculate_streak(&chain, &links);

                println!("Information for chain named \"{}\"", chain.name);
                println!("Current streak: {}", streak.streak);
                println!("Longest streak: {}", streak.longest_streak);
                println!("");
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("add-chain") {
        let chain_name = matches.value_of("chain").unwrap();

        if matches.is_present("all") {
            let chain = Chain {
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

            database::add_chain(&conn, &chain)?;

            println!(
                "Created a chain named \"{}\" with a filter of \"all\"",
                chain_name
            );
        } else if matches.is_present("weekday") {
            let chain = Chain {
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

            database::add_chain(&conn, &chain)?;

            println!(
                "Created a chain named \"{}\" with a filter of \"weekday\"",
                chain_name
            );
        } else if matches.is_present("weekend") {
            let chain = Chain {
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

            database::add_chain(&conn, &chain)?;

            println!(
                "Created a chain named \"{}\" with a filter of \"weekend\"",
                chain_name
            );
        } else if matches.is_present("custom") {
            todo!()
        } else {
            let chain = Chain {
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

            database::add_chain(&conn, &chain)?;

            println!(
                "Created a chain named \"{}\" with a filter of \"all\"",
                chain_name
            );
        }
    }

    if let Some(matches) = matches.subcommand_matches("delete-chain") {
        let chain_name = matches.value_of("chain").unwrap();
        let chain_id = database::get_chain_id_for_name(&conn, &chain_name)?;
        let links = database::get_links_for_chain_id(&conn, chain_id)?;

        for link in links.iter() {
            database::delete_link(&conn, &link)?;
        }

        database::delete_chain_for_name(&conn, &chain_name)?;
    }

    if let Some(matches) = matches.subcommand_matches("add-link") {
        let chain_name = matches.value_of("chain").unwrap();
        let date: NaiveDate;
        if matches.is_present("date") {
            date = NaiveDate::parse_from_str(matches.value_of("date").unwrap(), "%Y-%m-%d")?;
        } else {
            date = Utc::today().naive_utc();
        }

        let chain_id = match database::get_chain_id_for_name(&conn, &chain_name) {
            Ok(id) => id,
            Err(e) => return Err(e), // TODO: Make error handling more robust, or at least clean up the message
        };

        let link = Link { chain_id, date };
        let chain = database::get_chain_for_id(&conn, chain_id)?;

        if !logic::check_link(&chain, &link) {
            bail!(
                "{} is not a valid day based on the chain's filter",
                date.format("%Y-%m-%d")
            );
        }

        database::add_link(&conn, &link)?;

        println!(
            "Added link for \"{}\" to chain \"{}\"",
            date.format("%Y-%m-%d"),
            chain_name
        );
    }

    if let Some(matches) = matches.subcommand_matches("delete-link") {
        let date = NaiveDate::parse_from_str(matches.value_of("date").unwrap(), "%Y-%m-%d")?;
        let chain_name = matches.value_of("chain").unwrap();
        let chain_id = database::get_chain_id_for_name(&conn, &chain_name)?;

        let link = Link { chain_id, date };

        database::delete_link(&conn, &link)?;
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
    //    let mut date = Utc::today().naive_utc();
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
