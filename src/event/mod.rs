mod cyclicity;
pub mod occurrence;
mod period;

use crate::chrono::DateTimeDef;
use chrono::{Date, Utc};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

pub use self::{cyclicity::EventCyclicity, occurrence::EventOccurrence, period::EventPeriod};

use optfield::optfield;

#[optfield(
    pub EventPartial,
    merge_fn = pub,
    attrs,
)]
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Event {
    name: String,
    description: Option<String>,
    period: EventPeriod,
    cyclicity: Option<EventCyclicity>,
    exceptions: Vec<DateTimeDef>,
}

impl Event {
    pub fn get_occurrence_at(&self, date: Date<Utc>) -> Option<EventOccurrence> {
        use EventCyclicity::*;

        let period = if self.period.contains(date) {
            self.period.clone()
        } else if self.may_any_next_period_contains(date) {
            let new_start: Option<EventPeriod> = match self.cyclicity.as_ref().unwrap() {
                EveryDay => Some(self.period.same_with_new_start_day(date)),
                EveryWeek => self.period.same_with_new_start_week(date),
                EveryMonth => self.period.same_with_new_start_month(date),
                EveryYear => self.period.same_with_new_start_year(date),
                Custom(_) => None,
            };

            if new_start.is_none() {
                return None;
            }

            new_start.unwrap()
        } else {
            return None;
        };

        let occurrence = self.create_occurrence(&period);

        for exception in &self.exceptions {
            if exception.0 == occurrence.period.get_date_time_start() {
                return None;
            }
        }

        return Some(occurrence);
    }

    fn may_any_next_period_contains(&self, date: Date<Utc>) -> bool {
        self.cyclicity.is_some() && self.period.starts_before(date)
    }

    fn create_occurrence(&self, period: &EventPeriod) -> EventOccurrence {
        return EventOccurrence {
            name: self.name.clone(),
            description: self.description.clone(),
            period: period.clone(),
        };
    }
}

impl EventPartial {
    bind_partial_filler_default!(new, name);

    bind_partial_filler!(with_description, description);
    bind_partial_filler!(with_period, period, EventPeriod);
    bind_partial_filler!(with_cyclicity, cyclicity, EventCyclicity);

    pub fn complete(self) -> Event {
        let mut event: Event = Default::default();
        event.merge_opt(self);
        event
    }
}
