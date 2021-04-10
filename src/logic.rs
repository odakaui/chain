use crate::{Chain, Link, Streak};
use chrono::NaiveDate;

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
