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

    pub fn day(&self, date: Date<Utc>) -> Vec<EventOccurance> {
        self.events
            .iter()
            .filter_map(|event| event.get_occurence_at(date))
            .collect()
    }
}

#[macro_use]
extern crate optional_struct;

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
    pub fn get_occurence_at(&self, date: Date<Utc>) -> Option<EventOccurance> {
        if self.period.contains(date) {
            return Some(self.create_occurence(&self.period));
        }
        if let Some(cyclicity) = &self.cyclicity {
            if self.period.starts_before(date) {
                use EventCyclicity::*;

                match cyclicity {
                    EveryDay => {
                        return Some(
                            self.create_occurence(&self.period.same_with_new_start_day(date)),
                        );
                    }
                    EveryWeek => {
                        let new_start = &self.period.same_with_new_start_week(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurence(&new_start));
                        }
                    }
                    EveryMonth => {
                        let new_start = &self.period.same_with_new_start_month(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurence(&new_start));
                        }
                    }
                    EveryYear => {
                        let new_start = &self.period.same_with_new_start_year(date);
                        if let Some(new_start) = new_start {
                            return Some(self.create_occurence(&new_start));
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

    fn create_occurence(&self, period: &EventPeriod) -> EventOccurance {
        return EventOccurance {
            name: self.name.clone(),
            description: self.description.clone(),
            period: period.clone(),
        };
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EventOccurance {
    name: String,
    description: Option<String>,
    period: EventPeriod,
}

#[derive(Debug)]
pub enum EventCyclicity {
    EveryDay,
    EveryWeek,
    EveryMonth,
    EveryYear,
    Custom(EventCustomCyclicity),
}

#[derive(Debug)]
pub enum EventCustomCyclicity {
    EveryNDays {
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: Vec<Date<Utc>>,
    },
    EveryNWeeks {
        table: EventWeekCyclicityTable,
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: Vec<Date<Utc>>,
    },
    EveryNMonths {
        kind: EventMonthCyclicityType,
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: Vec<Date<Utc>>,
    },
    EveryNYears {
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: Vec<Date<Utc>>,
    },
}

#[derive(Debug)]
pub enum EventCyclicityEnd {
    OnDay(Date<Utc>),
    AfterNOccurences(u64),
}

#[derive(Debug)]
pub struct EventWeekCyclicityTable {
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    satuday: bool,
    sunday: bool,
}

#[derive(Debug)]
pub enum EventMonthCyclicityType {
    WeekDay,
    MonthDay,
}

use chrono::{Date, DateTime, Duration, NaiveDate, Utc};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EventPeriod {
    StartEnd(DateTime<Utc>, DateTime<Utc>),
    WholeDays(Date<Utc>, Date<Utc>),
}

impl EventPeriod {
    fn contains(&self, date: Date<Utc>) -> bool {
        let period = self.clone();

        match period {
            Self::StartEnd(start, end) => {
                let date = date.and_time(Utc::now().time()).unwrap();
                !((date - start).num_milliseconds() < 0 || (end - date).num_milliseconds() < 0)
            }
            Self::WholeDays(start, end) => {
                !((date - start).num_milliseconds() < 0 || (end - date).num_milliseconds() < 0)
            }
        }
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        let period = self.clone();

        match period {
            Self::StartEnd(start, _) => {
                let date = date.and_time(Utc::now().time()).unwrap();
                (start - date).num_milliseconds() < 0
            }
            Self::WholeDays(start, _) => (start - date).num_milliseconds() < 0,
        }
    }

    fn same_with_new_start_day(&self, new_date: Date<Utc>) -> Self {
        let period = self.clone();

        match period {
            Self::StartEnd(start, end) => {
                let total_duration = end - start;
                let time_at_start = start.time();

                let new_date = new_date.and_time(time_at_start).unwrap();

                Self::StartEnd(new_date, new_date + total_duration)
            }
            Self::WholeDays(start, end) => {
                let total_duration = end - start;
                Self::WholeDays(new_date, new_date + total_duration)
            }
        }
    }

    fn same_with_new_start_week(&self, new_date: Date<Utc>) -> Option<Self> {
        let period = self.clone();
        use chrono::Datelike;

        let (starting_weekday, ending_weekday) = match period {
            Self::StartEnd(start, end) => (
                start.date().weekday().number_from_monday(),
                end.date().weekday().number_from_monday(),
            ),
            Self::WholeDays(start, end) => (
                start.weekday().number_from_monday(),
                end.weekday().number_from_monday(),
            ),
        };

        match new_date.weekday().number_from_monday() {
            value if value >= starting_weekday && value <= ending_weekday => {
                let day_difference = (value - starting_weekday) as i64;
                let new_date = new_date - Duration::days(day_difference);

                Some(self.same_with_new_start_day(new_date))
            }
            _ => None,
        }
    }

    fn same_with_new_start_month(&self, new_date: Date<Utc>) -> Option<Self> {
        let period = self.clone();
        use chrono::Datelike;

        let (starting_monthday, ending_monthday) = match period {
            Self::StartEnd(start, end) => (start.date().day(), end.date().day()),
            Self::WholeDays(start, end) => (start.day(), end.day()),
        };

        match new_date.day() {
            value if value >= starting_monthday && value <= ending_monthday => {
                let day_difference = (value - starting_monthday) as i64;
                let new_date = new_date - Duration::days(day_difference);

                Some(self.same_with_new_start_day(new_date))
            }
            _ => None,
        }
    }

    fn same_with_new_start_year(&self, new_date: Date<Utc>) -> Option<Self> {
        let period = self.clone();
        use chrono::Datelike;

        let ((starting_month, starting_monthday), (ending_month, ending_monthday), day_difference) =
            match period {
                Self::StartEnd(start, end) => (
                    (start.date().month(), start.date().day()),
                    (end.date().month(), end.date().day()),
                    new_date
                        - Date::from_utc(
                            NaiveDate::from_ymd(
                                start.date().year(),
                                new_date.month(),
                                start.date().day(),
                            ),
                            Utc,
                        ),
                ),
                Self::WholeDays(start, end) => (
                    (start.month(), start.day()),
                    (end.month(), end.day()),
                    new_date
                        - Date::from_utc(
                            NaiveDate::from_ymd(start.year(), new_date.month(), start.day()),
                            Utc,
                        ),
                ),
            };

        if new_date.day() < starting_monthday || new_date.day() > ending_monthday {
            return None;
        }

        let day_difference = day_difference.num_days();

        match new_date.month() {
            value if value >= starting_month && value <= ending_month => {
                let new_date = new_date - Duration::days(day_difference);

                Some(self.same_with_new_start_day(new_date))
            }
            _ => None,
        }
    }
}

impl Default for EventPeriod {
    fn default() -> Self {
        Self::WholeDays(Utc::today(), Utc::today() + Duration::days(1))
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn one_day_and_one_time_date() {
        let calendar = Calendar::new();

        let today = Utc::today();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        let event = EventPartial::new(String::from("Date"))
            .with_period(EventPeriod::WholeDays(today, today))
            .complete();

        let calendar = calendar.add_event(event);

        let expected_occurance = vec![EventOccurance {
            name: "Date".into(),
            description: None,
            period: EventPeriod::WholeDays(today, today),
        }];

        assert_ne!(calendar.day(yesterday), expected_occurance);
        assert_eq!(calendar.day(today), expected_occurance);
        assert_ne!(calendar.day(tomorrow), expected_occurance);
    }

    #[test]
    fn one_hour_and_one_time_date() {
        let calendar = Calendar::new();

        let today = Utc::today();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        let (date_start, date_end) = (Utc::now(), Utc::now() + Duration::hours(1));
        let event = EventPartial::new(String::from("Date"))
            .with_period(EventPeriod::StartEnd(date_start, date_end))
            .complete();

        let calendar = calendar.add_event(event);

        let expected_occurance = vec![EventOccurance {
            name: "Date".into(),
            description: None,
            period: EventPeriod::StartEnd(date_start, date_end),
        }];

        assert_ne!(calendar.day(yesterday), expected_occurance);
        assert_eq!(calendar.day(today), expected_occurance);
        assert_ne!(calendar.day(tomorrow), expected_occurance);
    }
}
