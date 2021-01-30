use crate::structs::{Chain, Streak};

pub fn print_streak(streak: &Streak) {
    println!("Current streak: {}", streak.streak);
    println!("Longest streak: {}", streak.longest_streak);
}

pub fn print_chain_name(chain: &Chain) {
    println!("{}", chain.name);
}
