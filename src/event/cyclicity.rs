use chrono::{Date, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_support")]
use crate::serde::{from_date_into_string, from_string_into_date};

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum EventCyclicity {
    EveryDay,
    EveryWeek,
    EveryMonth,
    EveryYear,
    Custom(EventCustomCyclicity),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum EventCustomCyclicity {
    EveryNDays {
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: DateDef,
    },
    EveryNWeeks {
        table: EventWeekCyclicityTable,
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: DateDef,
    },
    EveryNMonths {
        kind: EventMonthCyclicityType,
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: DateDef,
    },
    EveryNYears {
        n: u64,
        ends: Option<EventCyclicityEnd>,
        exceptions: DateDef,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum EventCyclicityEnd {
    OnDay(
        #[cfg_attr(
            feature = "serde_support",
            serde(
                serialize_with = "from_date_into_string",
                deserialize_with = "from_string_into_date"
            )
        )]
        Date<Utc>,
    ),
    AfterNOccurrences(u64),
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub enum EventMonthCyclicityType {
    WeekDay,
    MonthDay,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DateDef(
    #[cfg_attr(
        feature = "serde_support",
        serde(
            serialize_with = "from_date_into_string",
            deserialize_with = "from_string_into_date"
        )
    )]
    Date<Utc>,
);
