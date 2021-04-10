use chrono::NaiveDate;

#[derive(Debug)]
pub struct Chain {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct Link {
    pub chain_id: i32,
    pub date: NaiveDate,
}

#[derive(Debug)]
pub struct Streak {
    pub name: String,
    pub streak: i32,
    pub longest_streak: i32,
}
