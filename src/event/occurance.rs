use super::period::EventPeriod;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EventOccurance {
    pub name: String,
    pub description: Option<String>,
    pub period: EventPeriod,
}
