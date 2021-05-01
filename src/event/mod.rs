mod cyclicity;
pub mod occurrence;
mod period;

use chrono::{Date, Utc};

pub use self::{cyclicity::EventCyclicity, occurrence::EventOccurrence, period::EventPeriod};

#[derive(OptionalStruct, Debug, Default)]
#[optional_name = "EventPartial"]
#[optional_derive(Debug, Default)]
pub struct Event {
    name: String,
    description: Option<String>,
    period: EventPeriod,
    cyclicity: Option<EventCyclicity>,
}

impl Event {
    pub fn get_occurrence_at(&self, date: Date<Utc>) -> Option<EventOccurrence> {
        if self.period.contains(date) {
            return Some(self.create_occurrence(&self.period));
        }

        if let Some(cyclicity) = &self.cyclicity {
            if self.period.starts_before(date) {
                use EventCyclicity::*;

                match cyclicity {
                    EveryDay => {
                        return Some(
                            self.create_occurrence(&self.period.same_with_new_start_day(date)),
                        );
                    }
                    EveryWeek => {
                        let new_start = &self.period.same_with_new_start_week(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurrence(&new_start));
                        }
                    }
                    EveryMonth => {
                        let new_start = &self.period.same_with_new_start_month(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurrence(&new_start));
                        }
                    }
                    EveryYear => {
                        let new_start = &self.period.same_with_new_start_year(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurrence(&new_start));
                        }
                    }
                    Custom(_) => {
                        unimplemented!()
                    }
                }
            }
        }
        None
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
    pub fn new<T: ToString>(name: T) -> Self {
        let name = Some(name.to_string());
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_description<T: ToString>(self, description: T) -> Self {
        let description = Some(description.to_string());

        Self {
            description,
            ..self
        }
    }

    pub fn with_period(self, period: EventPeriod) -> Self {
        let period = Some(period);

        Self { period, ..self }
    }

    pub fn with_cyclicity(self, cyclicity: EventCyclicity) -> Self {
        let cyclicity = Some(cyclicity);

        Self { cyclicity, ..self }
    }

    pub fn complete(self) -> Event {
        let mut event: Event = Default::default();
        event.apply_options(self);
        event
    }
}
