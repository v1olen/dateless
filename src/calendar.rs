use super::event::{occurrence::EventOccurrence, Event};
use chrono::{Date, Utc};
#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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

    /**
       Adds passed event to the instance.

       # Examples

       ```rust
       use dateless::prelude::*;
       use chrono::Utc;

       let mut calendar = Calendar::new();

       calendar.add_event(
           EventPartial::new(String::from("Anne's birthday"))
               .whole_days(Utc::today(), Utc::today())
               .complete()
       );
       ```
    */
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /**
       Returns `Vec` of `EventOccurrence`s for the given day.

       # Examples

       ```rust
       use dateless::prelude::*;
       use chrono::Utc;

       let mut calendar = Calendar::new();

       calendar.add_event(
           EventPartial::new(String::from("Anne's birthday"))
               .whole_days(Utc::today(), Utc::today())
               .complete()
       );

       let events_today = calendar.day(Utc::today());
       ```
    */
    pub fn day(&self, date: Date<Utc>) -> Vec<EventOccurrence> {
        self.events
            .iter()
            .filter_map(|event| event.get_occurrence_at(date))
            .collect()
    }
}
