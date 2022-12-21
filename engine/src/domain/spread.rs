#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Spread(pub Option<i64>, pub Option<i64>);

impl Spread {
    pub fn overlaping(&self) -> bool {
        match (self.0, self.1) {
            (Some(b), Some(s)) => b >= s,
            _ => false,
        }
    }
}
