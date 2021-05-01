use chrono::{Date, Utc};

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
    AfterNOccurrences(u64),
}

#[derive(Debug)]
pub struct EventWeekCyclicityTable {
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
}

#[derive(Debug)]
pub enum EventMonthCyclicityType {
    WeekDay,
    MonthDay,
}
