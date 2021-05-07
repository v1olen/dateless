use chrono::{Date, DateTime, Duration, NaiveDate, NaiveTime, Utc};

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

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Period for StartEnd {
    fn contains(&self, date: Date<Utc>) -> bool {
        {
            let date = date.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap();
            if !((self.0 - date).num_milliseconds() >= 0 || (self.1 - date).num_milliseconds() > 0)
            {
                return false;
            }
        }
        let date = (date + Duration::days(1))
            .and_time(Utc::now().time())
            .unwrap();
        (self.0 - date).num_milliseconds() < 0
    }

    fn get_date_time_start(&self) -> DateTime<Utc> {
        self.0.clone()
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        let date = date.and_time(Utc::now().time()).unwrap();
        (self.0 - date).num_milliseconds() < 0
    }

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn Period> {
        let total_duration = self.1 - self.0;
        let time_at_start = self.0.time();

        let date = date.and_time(time_at_start).unwrap();

        Box::new(Self(date, date + total_duration))
    }

    fn as_weekdays(&self) -> (u32, u32) {
        use chrono::Datelike;

        (
            self.0.date().weekday().number_from_monday(),
            self.1.date().weekday().number_from_monday(),
        )
    }

    fn as_days_of_month(&self) -> (u32, u32) {
        use chrono::Datelike;

        (self.0.day(), self.1.day())
    }

    fn as_months(&self) -> (u32, u32) {
        use chrono::Datelike;

        (self.0.date().day(), self.1.date().day())
    }

    fn with_new_month(&self, month: u32) -> Date<Utc> {
        use chrono::Datelike;

        Date::from_utc(NaiveDate::from_ymd(self.0.year(), month, self.0.day()), Utc)
    }

    fn cloned(&self) -> Box<dyn Period> {
        Box::new(self.clone())
    }
}
