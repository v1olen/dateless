use super::period::EventPeriod;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(PartialEq))]
pub struct EventOccurrence {
    pub name: String,
    pub description: Option<String>,
    pub period: EventPeriod,
}
