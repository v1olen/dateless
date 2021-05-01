use super::event::{occurrence::EventOccurrence, Event};
use chrono::{Date, Utc};

#[derive(Debug, Default)]
pub struct Calendar {
    events: Vec<Event>,
}

impl Calendar {
    /**
       Creates new empty `Calendar` instance. It is equivalent to:

       ```rust
       # use dateless::Calendar;
       let instance: Calendar = Default::default();
       ```

       # Examples

       ```rust
       use dateless::Calendar;

       let calendar: Calendar = Default::default();
       ```
    */
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn day(&self, date: Date<Utc>) -> Vec<EventOccurrence> {
        self.events
            .iter()
            .filter_map(|event| event.get_occurrence_at(date))
            .collect()
    }
}
