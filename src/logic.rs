use super::{Chain, Day, Link, Streak};
use chrono::{Datelike, Local, NaiveDate, Duration};

use super::FORMAT;

pub fn calculate_streak(chain: &Chain, links: &Vec<Link>) -> Streak {
    let name = chain.name.to_string();

    let mut date: Option<NaiveDate> = None;
    let mut prev_date: Option<NaiveDate>;
    let mut streak = 0;
    let mut longest_streak = 0;

    for link in links.iter() {
        prev_date = date;
        date = Some(link.date);

        if date.is_some() && prev_date.is_some() {
            let between = date
                .unwrap()
                .signed_duration_since(prev_date.unwrap())
                .num_days()
                - 1;

            if between > 0 {
                if streak > longest_streak {
                    longest_streak = streak;
                }
                streak = 0;
            }
        }

        streak += 1;
    }

    if streak > longest_streak {
        longest_streak = streak;
    }

    Streak {
        name,
        streak,
        longest_streak,
    }
}

pub fn create_days(links: &Vec<Link>) -> Vec<Day> {
    let mut days: Vec<Day> = Vec::new();

    if links.len() == 0 {
        return create_dummy_days()
    }

    let start_date = links[0].date;
    let end_date = Local::today().naive_local(); 

    let difference = end_date.signed_duration_since(start_date).num_days();
    let start_date = if difference < 10 {
        start_date - Duration::days(9 - difference)
    } else {
        start_date + Duration::days(difference - 9) 
    };

    let links = if links.len() > 10 {
        links.get(links.len() - 10..).unwrap()
    } else {
        links.get(..).unwrap()
    };

    assert!(start_date.iter_days().take(10).last().unwrap().format(FORMAT).to_string() == end_date.format(FORMAT).to_string());

    let mut is_done = false;

    for date in start_date.iter_days().take(10) {
        for link in links {
            if link.date.signed_duration_since(date).num_days() == 0 {
                is_done = true;
            }
        }

        let day = Day {
            day: date.day() as i32,
            is_done: is_done
        };

        days.push(day);
        
        is_done = false;
    }

    assert!(days.len() == 10);

    days
}

pub fn create_dummy_days() -> Vec<Day> {
    let mut days: Vec<Day> = Vec::new();

    let today = Local::today().naive_local(); 
    let start_date = today - Duration::days(9);

    for date in start_date.iter_days().take(10) {
        let day = Day {
            day: date.day() as i32,
            is_done: false
        };

        days.push(day);
    }

    assert!(days.len() == 10);

    days
}
