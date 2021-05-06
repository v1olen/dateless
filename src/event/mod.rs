mod cyclicity;
pub mod occurrence;
mod period;

use crate::chrono::DateTimeDef;
use chrono::{Date, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

pub use self::{
    cyclicity::EventCyclicity,
    cyclicity::{AnnualCycle, DailyCycle, MonthlyCycle, WeeklyCycle},
    occurrence::EventOccurrence,
    period::EventPeriod,
};

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
    cyclicity: Option<Box<dyn EventCyclicity>>,
    exceptions: Vec<DateTimeDef>,
}

impl Event {
    pub fn get_occurrence_at(&self, date: Date<Utc>) -> Option<EventOccurrence> {
        let period = if self.period.contains(date) {
            self.period.clone()
        } else if self.may_any_next_period_contains(date) {
            let new_start: Option<EventPeriod> = match &self.cyclicity {
                Some(value) => value.same_period_at(self.period.clone(), date),
                _ => return None,
            };

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

    bind_partial_filler_boxed!(with_cyclicity, cyclicity, EventCyclicity);

    bind_partial_filler_cyclicity!(daily, DailyCycle);
    bind_partial_filler_cyclicity!(weekly, WeeklyCycle);
    bind_partial_filler_cyclicity!(monthly, MonthlyCycle);
    bind_partial_filler_cyclicity!(annual, AnnualCycle);

    pub fn complete(self) -> Event {
        let mut event: Event = Default::default();
        event.merge_opt(self);
        event
    }
}
