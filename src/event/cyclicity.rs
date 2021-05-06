use chrono::Utc;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde_support", typetag::serde(tag = "type"))]
pub trait EventCyclicity: std::fmt::Debug {
    fn same_period_at(
        &self,
        same_period: super::EventPeriod,
        at_date: chrono::Date<Utc>,
    ) -> Option<super::EventPeriod>;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DailyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl EventCyclicity for DailyCycle {
    fn same_period_at(
        &self,
        same_period: super::EventPeriod,
        at_date: chrono::Date<Utc>,
    ) -> Option<super::EventPeriod> {
        Some(same_period.same_with_new_start_day(at_date))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct WeeklyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl EventCyclicity for WeeklyCycle {
    fn same_period_at(
        &self,
        same_period: super::EventPeriod,
        at_date: chrono::Date<Utc>,
    ) -> Option<super::EventPeriod> {
        same_period.same_with_new_start_week(at_date)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct MonthlyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl EventCyclicity for MonthlyCycle {
    fn same_period_at(
        &self,
        same_period: super::EventPeriod,
        at_date: chrono::Date<Utc>,
    ) -> Option<super::EventPeriod> {
        same_period.same_with_new_start_month(at_date)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct AnnualCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl EventCyclicity for AnnualCycle {
    fn same_period_at(
        &self,
        same_period: super::EventPeriod,
        at_date: chrono::Date<Utc>,
    ) -> Option<super::EventPeriod> {
        same_period.same_with_new_start_year(at_date)
    }
}

// #[cfg(feature = "serde_support")]
// use crate::serde::{from_date_into_string, from_string_into_date};

// #[derive(Debug)]
// #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
// pub enum EventCustomCyclicity {
//     EveryNDays {
//         n: u64,
//         ends: Option<EventCyclicityEnd>,
//     },
//     EveryNWeeks {
//         table: EventWeekCyclicityTable,
//         n: u64,
//         ends: Option<EventCyclicityEnd>,
//     },
//     EveryNMonths {
//         kind: EventMonthCyclicityType,
//         n: u64,
//         ends: Option<EventCyclicityEnd>,
//     },
//     EveryNYears {
//         n: u64,
//         ends: Option<EventCyclicityEnd>,
//     },
// }

// #[derive(Debug)]
// #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
// pub enum EventCyclicityEnd {
//     OnDay(
//         #[cfg_attr(
//             feature = "serde_support",
//             serde(
//                 serialize_with = "from_date_into_string",
//                 deserialize_with = "from_string_into_date"
//             )
//         )]
//         Date<Utc>,
//     ),
//     AfterNOccurrences(u64),
// }

// #[derive(Debug)]
// #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
// pub struct EventWeekCyclicityTable {
//     monday: bool,
//     tuesday: bool,
//     wednesday: bool,
//     thursday: bool,
//     friday: bool,
//     saturday: bool,
//     sunday: bool,
// }

// #[derive(Debug)]
// #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
// pub enum EventMonthCyclicityType {
//     WeekDay,
//     MonthDay,
// }
