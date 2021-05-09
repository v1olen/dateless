use chrono::{Date, Utc};
use std::fmt::Debug;

use super::Period;

#[cfg_attr(feature = "serde_support", typetag::serde(tag = "type"))]
pub trait Cyclicity: Debug + Send {
    fn same_period_at(
        &self,
        same_period: Box<dyn Period>,
        at_date: Date<Utc>,
    ) -> Option<Box<dyn Period>>;
}

mod annual;
mod daily;
mod monthly;
mod weekly;

pub use annual::*;
pub use daily::*;
pub use monthly::*;
pub use weekly::*;
