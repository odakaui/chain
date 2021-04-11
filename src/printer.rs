use super::structs::{Chain, Day, Link, Streak};

pub fn print_days(days: &Vec<Day>) {
    if days.len() == 0 {
        return
    }
    
    let mut dates = String::new();
    let mut is_done = String::new();

    for day in days.iter() {
        dates.push_str(&format!("{:02} ", day.day));
        is_done.push_str(&format!("{:>2} ", if day.is_done { "XX" } else { "--" }));
    }

    println!("{}", dates);
    println!("{}", is_done);
    println!("");
}

pub fn print_streaks(streaks: &Vec<(Streak, Vec<Day>)>) {
    if streaks.len() > 0 {
        for (streak, days) in streaks.iter() {
            println!("{}", streak.name);
            println!("Current streak: {}", streak.streak);
            println!("Longest streak: {}", streak.longest_streak);

            print_days(&days);
        }
    } else {
        println!("Congratulations. You completed all of your chains for today.");
    }
}

pub fn print_streak(streak: &Streak, days: &Vec<Day>) {
    println!("{}", streak.name);
    println!("Current streak: {}", streak.streak);
    println!("Longest streak: {}", streak.longest_streak);

    print_days(days);
}

pub fn print_streaks_machine(streaks: &Vec<(Streak, Vec<Day>)>) {
    println!("{}", streaks.len());
}

pub fn print_add(chain: &Chain, link: &Link) {
    println!(
        "Added link for \"{}\" to \"{}\"",
        link.date.format("%Y-%m-%d"),
        chain.name
    );
}

pub fn print_mv(chain: &Chain, current: &Link, new: &Link) {
    println!(
        "Update link from \"{}\" to \"{}\" for \"{}\"",
        current.date.format("%Y-%m-%d"),
        new.date.format("%Y-%m-%d"),
        chain.name
    );
}

pub fn print_rm(chain: &Chain, link: &Link) {
    println!(
        "Deleted link for \"{}\" from \"{}\"",
        link.date.format("%Y-%m-%d"),
        chain.name
    );
}

pub fn print_add_chain(chain: &Chain) {
    println!("Added \"{}\"", chain.name);
}

pub fn print_rename_chain(chain: &Chain, new: &str) {
    println!("Renamed \"{}\" to \"{}\"", &chain.name, new);
}

pub fn print_rm_chain(chain: &Chain) {
    println!("Deleted \"{}\"", &chain.name);
}

pub fn print_ls(chains: &Vec<Chain>) {
    for chain in chains.iter() {
        println!("{}", chain.name);
    }
}

