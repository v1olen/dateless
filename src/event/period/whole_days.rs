use chrono::{Date, DateTime, NaiveDate, NaiveTime, Utc};

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

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Period for WholeDays {
    fn contains(&self, date: Date<Utc>) -> bool {
        !((date - self.0).num_milliseconds() < 0 || (self.1 - date).num_milliseconds() < 0)
    }

    fn get_date_time_start(&self) -> DateTime<Utc> {
        self.0.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap()
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        (self.0 - date).num_milliseconds() < 0
    }

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn Period> {
        let total_duration = self.1 - self.0;
        Box::new(Self(date, date + total_duration))
    }

    fn as_weekdays(&self) -> (u32, u32) {
        use chrono::Datelike;

        (
            self.0.weekday().number_from_monday(),
            self.1.weekday().number_from_monday(),
        )
    }

    fn as_days_of_month(&self) -> (u32, u32) {
        use chrono::Datelike;

        (self.0.day(), self.1.day())
    }

    fn as_months(&self) -> (u32, u32) {
        use chrono::Datelike;

        (self.0.month(), self.1.month())
    }

    fn with_new_month(&self, month: u32) -> Date<Utc> {
        use chrono::Datelike;

        Date::from_utc(NaiveDate::from_ymd(self.0.year(), month, self.0.day()), Utc)
    }

    fn cloned(&self) -> Box<dyn Period> {
        Box::new(self.clone())
    }
}
