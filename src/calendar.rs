use super::event::{Event, occurrence::EventOccurrence};
use chrono::{Date, Utc};

#[derive(Debug, Default)]
pub struct Calendar {
    events: Vec<Event>,
}

impl Calendar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_event(mut self, event: Event) -> Self {
        &self.events.push(event);
        self
    }

    pub fn day(&self, date: Date<Utc>) -> Vec<EventOccurrence> {
        self.events
            .iter()
            .filter_map(|event| event.get_occurrence_at(date))
            .collect()
    }
}
