use chrono::Utc;

#[cfg_attr(feature = "serde_support", typetag::serde(tag = "type"))]
pub trait Cyclicity: std::fmt::Debug {
    fn same_period_at(
        &self,
        same_period: Box<dyn super::Period>,
        at_date: chrono::Date<Utc>,
    ) -> Option<Box<dyn super::Period>>;
}

mod annual;
mod daily;
mod monthly;
mod weekly;

pub use annual::*;
pub use daily::*;
pub use monthly::*;
pub use weekly::*;
