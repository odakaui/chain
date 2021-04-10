use crate::structs::{Chain, Link, Streak};

pub fn print_streaks(streaks: &Vec<Streak>) {
    if streaks.len() > 0 {
        for streak in streaks.iter() {
            println!("{}", streak.name);
            println!("Current streak: {}", streak.streak);
            println!("Longest streak: {}", streak.longest_streak);
            println!("");
        }
    } else {
        println!("Congratulations. You completed all of your chains for today.");
    }
}

pub fn print_streak(streak: &Streak) {
    println!("{}", streak.name);
    println!("Current streak: {}", streak.streak);
    println!("Longest streak: {}", streak.longest_streak);
}

pub fn print_streaks_machine(streaks: &Vec<Streak>) {
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
    println!("Update link from \"{}\" to \"{}\" for \"{}\"", current.date.format("%Y-%m-%d"), new.date.format("%Y-%m-%d"), chain.name);
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
