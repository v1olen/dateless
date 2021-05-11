use chrono::{Date, DateTime, Utc};
use std::fmt::Debug;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_support", typetag::serde(tag = "type"))]
pub trait Period: Debug + Send {
    fn contains(&self, date: Date<Utc>) -> bool;
    fn get_date_time_start(&self) -> DateTime<Utc>;
    fn starts_before(&self, date: Date<Utc>) -> bool;
    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn Period>;
    // Todo: Create macro for cloned
    fn cloned(&self) -> Box<dyn Period>;
    fn as_weekdays(&self) -> (u32, u32);
    fn as_days_of_month(&self) -> (u32, u32);
    fn as_months(&self) -> (u32, u32);
    fn with_new_month(&self, month: u32) -> Date<Utc>;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct PeriodDef(pub Box<dyn Period>);

impl Default for PeriodDef {
    fn default() -> Self {
        Self(Box::new(WholeDays(
            Utc::today(),
            Utc::today() + chrono::Duration::days(1),
        )))
    }
}

pub trait WithBoundaries<T> {
    fn start(&self) -> T;
    fn end(&self) -> T;
}

mod start_end;
mod whole_days;

pub use start_end::*;
pub use whole_days::*;
