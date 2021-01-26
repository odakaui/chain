use crate::Chain;
use crate::Link;
use chrono::{Datelike, Weekday};

pub fn check_link(chain: &Chain, link: &Link) -> bool {
    let weekday = link.date.weekday();

    is_valid(chain, &weekday)
}

pub fn is_valid(chain: &Chain, weekday: &Weekday) -> bool {
    match weekday {
        Weekday::Sun => chain.sunday,
        Weekday::Mon => chain.monday,
        Weekday::Tue => chain.tuesday,
        Weekday::Wed => chain.wednesday,
        Weekday::Thu => chain.thursday,
        Weekday::Fri => chain.friday,
        Weekday::Sat => chain.saturday,
    }
}
