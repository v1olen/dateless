use chrono::{Date, DateTime, Duration, NaiveDate, NaiveTime, Utc};

#[cfg(feature = "serde_support")]
use chrono::serde::ts_seconds;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_support")]
use crate::serde::{from_date_into_string, from_string_into_date};

#[cfg_attr(feature = "serde_support", typetag::serde(tag = "type"))]
pub trait EventPeriod: std::fmt::Debug {
    fn contains(&self, date: Date<Utc>) -> bool;
    fn get_date_time_start(&self) -> DateTime<Utc>;
    fn starts_before(&self, date: Date<Utc>) -> bool;
    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn EventPeriod>;
    // Todo: Create macro for cloned
    fn cloned(&self) -> Box<dyn EventPeriod>;
    fn as_weekdays(&self) -> (u32, u32);
    fn as_days_of_month(&self) -> (u32, u32);
    fn as_months(&self) -> (u32, u32);
    fn with_new_month(&self, month: u32) -> Date<Utc>;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct EventPeriodDef(pub Box<dyn EventPeriod>);

impl Default for EventPeriodDef {
    fn default() -> Self {
        Self(Box::new(WholeDays(
            Utc::today(),
            Utc::today() + chrono::Duration::days(1),
        )))
    }
}

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
impl EventPeriod for WholeDays {
    fn contains(&self, date: Date<Utc>) -> bool {
        !((date - self.0).num_milliseconds() < 0 || (self.1 - date).num_milliseconds() < 0)
    }

    fn get_date_time_start(&self) -> DateTime<Utc> {
        self.0.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap()
    }

    fn starts_before(&self, date: Date<Utc>) -> bool {
        (self.0 - date).num_milliseconds() < 0
    }

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn EventPeriod> {
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

    fn cloned(&self) -> Box<dyn EventPeriod> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct StartEnd(
    #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] pub DateTime<Utc>,
    #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] pub DateTime<Utc>,
);

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl EventPeriod for StartEnd {
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

    fn with_new_start(&self, date: Date<Utc>) -> Box<dyn EventPeriod> {
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

    fn cloned(&self) -> Box<dyn EventPeriod> {
        Box::new(self.clone())
    }
}
