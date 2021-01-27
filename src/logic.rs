use crate::{logic, Chain, Link, Streak};
use chrono::{Datelike, NaiveDate, Weekday};

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

pub fn calculate_streak(chain: &Chain, links: &Vec<Link>) -> Streak {
    let mut date: Option<NaiveDate> = None;
    let mut prev_date: Option<NaiveDate>;
    let mut streak = 0;
    let mut longest_streak = 0;

    for link in links.iter() {
        prev_date = date;
        date = Some(link.date);

        streak += 1;

        if date.is_some() && prev_date.is_some() {
            let days = date
                .unwrap()
                .signed_duration_since(prev_date.unwrap())
                .num_days();
            let mut tmp_date = prev_date.unwrap().succ();

            for _ in 0..days - 1 {
                if logic::is_valid(&chain, &tmp_date.weekday()) {
                    if streak > longest_streak {
                        longest_streak = streak;
                        streak = 0;
                    }
                }

                tmp_date = tmp_date.succ();

                assert!(date.unwrap().signed_duration_since(tmp_date).num_days() >= 0);
                assert!(
                    tmp_date
                        .signed_duration_since(prev_date.unwrap())
                        .num_days()
                        >= 0
                );
            }
        }
    }

    if streak > longest_streak {
        longest_streak = streak;
    }

    Streak {
        streak,
        longest_streak,
    }
}
