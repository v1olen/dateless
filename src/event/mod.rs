mod cyclicity;
pub mod occurrence;
mod period;

use crate::chrono::DateTimeDef;
use chrono::{Date, DateTime, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

pub use self::period::PeriodDef;
pub use self::{cyclicity::Cyclicity, occurrence::EventOccurrence, period::Period};

use cyclicity::*;

pub use period::{StartEnd, WholeDays};

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
    cyclicity: Option<Box<dyn Cyclicity>>,
    exceptions: Vec<DateTimeDef>,
    period: PeriodDef,
}

impl Event {
    pub fn get_occurrence_at(&self, date: Date<Utc>) -> Option<EventOccurrence> {
        let period = if self.period.0.contains(date) {
            self.period.0.cloned()
        } else if self.may_any_next_period_contains(date) {
            let new_start: Option<Box<dyn Period>> = match &self.cyclicity {
                Some(value) => value.same_period_at(self.period.0.cloned(), date),
                _ => return None,
            };

            if new_start.is_none() {
                return None;
            }

            new_start.unwrap().cloned()
        } else {
            return None;
        };

        let occurrence = self.create_occurrence(period);

        for exception in &self.exceptions {
            if exception.0 == occurrence.period.0.get_date_time_start() {
                return None;
            }
        }

        return Some(occurrence);
    }

    fn may_any_next_period_contains(&self, date: Date<Utc>) -> bool {
        self.cyclicity.is_some() && self.period.0.starts_before(date)
    }

    fn create_occurrence(&self, period: Box<dyn Period>) -> EventOccurrence {
        return EventOccurrence {
            name: self.name.clone(),
            description: self.description.clone(),
            period: PeriodDef(period),
        };
    }
}

impl EventPartial {
    bind_partial_filler_default!(new, name);

    bind_partial_filler!(with_description, description);

    bind_partial_filler_boxed!(with_cyclicity, cyclicity, Cyclicity);

    bind_partial_trait_filler!(daily, DailyCycle, with_cyclicity);
    bind_partial_trait_filler!(weekly, WeeklyCycle, with_cyclicity);
    bind_partial_trait_filler!(monthly, MonthlyCycle, with_cyclicity);
    bind_partial_trait_filler!(annual, AnnualCycle, with_cyclicity);

    bind_partial_filler!(with_period, period, PeriodDef);

    pub fn from_to(self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.with_period(PeriodDef(Box::new(StartEnd(from, to))))
    }

    pub fn whole_days(self, from: Date<Utc>, to: Date<Utc>) -> Self {
        self.with_period(PeriodDef(Box::new(WholeDays(from, to))))
    }

    pub fn whole_day(self, from_to: Date<Utc>) -> Self {
        self.with_period(PeriodDef(Box::new(WholeDays(from_to, from_to))))
    }

    pub fn complete(self) -> Event {
        let mut event: Event = Default::default();
        event.merge_opt(self);
        event
    }
}
