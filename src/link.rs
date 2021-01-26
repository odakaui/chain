use chrono::NaiveDate;

#[derive(Debug)]
pub struct Link {
    pub chain_id: i32,
    pub date: NaiveDate,
}
