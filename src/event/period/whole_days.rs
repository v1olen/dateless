use chrono::{Date, DateTime, Datelike, NaiveDate, NaiveTime, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_support")]
use crate::serde::{from_date_into_string, from_string_into_date};

use super::Period;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct WholeDays(
    #[cfg_attr(
        feature = "serde_support",
        serde(
            serialize_with = "from_date_into_string",
            deserialize_with = "from_string_into_date"
        )
    )]
    pub Date<Utc>,
    #[cfg_attr(
        feature = "serde_support",
        serde(
            serialize_with = "from_date_into_string",
            deserialize_with = "from_string_into_date"
        )
    )]
    pub Date<Utc>,
);

impl_period_boundaries!(WholeDays, Date<Utc>);

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Period for WholeDays {
    fn contains(&self, date: Date<Utc>) -> bool {
        !((date - self.start()).num_milliseconds() < 0
            || (self.end() - date).num_milliseconds() < 0)
    }

    fn get_date_time_start(&self) -> DateTime<Utc> {
        self.start().and_time(NaiveTime::from_hms(0, 0, 0)).unwrap()
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        (self.start() - date).num_milliseconds() < 0
    }

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn Period> {
        let total_duration = self.end() - self.start();
        Box::new(Self(date, date + total_duration))
    }

    fn as_weekdays(&self) -> (u32, u32) {
        (
            self.start().weekday().number_from_monday(),
            self.end().weekday().number_from_monday(),
        )
    }

    fn as_days_of_month(&self) -> (u32, u32) {
        (self.start().day(), self.end().day())
    }

    fn as_months(&self) -> (u32, u32) {
        (self.start().month(), self.end().month())
    }

    fn with_new_month(&self, month: u32) -> Date<Utc> {
        Date::from_utc(
            NaiveDate::from_ymd(self.start().year(), month, self.start().day()),
            Utc,
        )
    }

    impl_cloned!(Period);
}
