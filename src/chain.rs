#[derive(Debug)]
pub struct Chain {
    pub id: Option<i32>,
    pub name: String,
    pub sunday: bool,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
}
