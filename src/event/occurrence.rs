use super::PeriodDef;
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct EventOccurrence {
    pub name: String,
    pub description: Option<String>,
    pub period: PeriodDef,
    pub origin: Uuid,
}
