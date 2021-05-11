use chrono::{Date, DateTime, Datelike, Duration, NaiveDate, NaiveTime, Utc};

#[cfg(feature = "serde_support")]
use chrono::serde::ts_seconds;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Period;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct StartEnd(
    #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] pub DateTime<Utc>,
    #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] pub DateTime<Utc>,
);

impl_period_boundaries!(StartEnd, DateTime<Utc>);

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Period for StartEnd {
    fn contains(&self, date: Date<Utc>) -> bool {
        {
            let date = date.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap();
            if !((self.start() - date).num_milliseconds() >= 0
                || (self.end() - date).num_milliseconds() > 0)
            {
                return false;
            }
        }
        let date = (date + Duration::days(1))
            .and_time(Utc::now().time())
            .unwrap();
        (self.start() - date).num_milliseconds() < 0
    }

    fn get_date_time_start(&self) -> DateTime<Utc> {
        self.start().clone()
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        let date = date.and_time(Utc::now().time()).unwrap();
        (self.start() - date).num_milliseconds() < 0
    }

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn Period> {
        let total_duration = self.end() - self.start();
        let time_at_start = self.start().time();

        let date = date.and_time(time_at_start).unwrap();

        Box::new(Self(date, date + total_duration))
    }

    fn as_weekdays(&self) -> (u32, u32) {
        (
            self.start().date().weekday().number_from_monday(),
            self.end().date().weekday().number_from_monday(),
        )
    }

    fn as_days_of_month(&self) -> (u32, u32) {
        (self.start().day(), self.end().day())
    }

    fn as_months(&self) -> (u32, u32) {
        (self.start().date().day(), self.end().date().day())
    }

    fn with_new_month(&self, month: u32) -> Date<Utc> {
        Date::from_utc(
            NaiveDate::from_ymd(self.start().year(), month, self.start().day()),
            Utc,
        )
    }

    fn cloned(&self) -> Box<dyn Period> {
        Box::new(self.clone())
    }
}
