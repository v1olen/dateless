use chrono::{Date, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Cyclicity;
use crate::event::period::Period;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DailyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Cyclicity for DailyCycle {
    fn same_period_at(
        &self,
        same_period: Box<dyn Period>,
        at_date: Date<Utc>,
    ) -> Option<Box<dyn Period>> {
        Some(same_period.with_new_start(at_date))
    }
}
