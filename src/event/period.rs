use chrono::{Date, DateTime, Duration, NaiveDate, NaiveTime, Utc};

#[cfg(feature = "serde_support")]
use chrono::serde::ts_seconds;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_support")]
use crate::serde::{from_date_into_string, from_string_into_date};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(test, derive(PartialEq))]
pub enum EventPeriod {
    StartEnd(
        #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] DateTime<Utc>,
        #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] DateTime<Utc>,
    ),
    WholeDays(
        #[cfg_attr(
            feature = "serde_support",
            serde(
                serialize_with = "from_date_into_string",
                deserialize_with = "from_string_into_date"
            )
        )]
        Date<Utc>,
        #[cfg_attr(
            feature = "serde_support",
            serde(
                serialize_with = "from_date_into_string",
                deserialize_with = "from_string_into_date"
            )
        )]
        Date<Utc>,
    ),
}

impl EventPeriod {
    pub fn contains(&self, date: Date<Utc>) -> bool {
        let period = self.clone();

        match period {
            Self::StartEnd(start, end) => {
                {
                    let date = date.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap();
                    if !((start - date).num_milliseconds() >= 0
                        || (end - date).num_milliseconds() > 0)
                    {
                        return false;
                    }
                }
                let date = (date + Duration::days(1))
                    .and_time(Utc::now().time())
                    .unwrap();
                (start - date).num_milliseconds() < 0
            }
            Self::WholeDays(start, end) => {
                !((date - start).num_milliseconds() < 0 || (end - date).num_milliseconds() < 0)
            }
        }
    }

    pub fn starts_before(&self, date: Date<Utc>) -> bool {
        let period = self.clone();

        match period {
            Self::StartEnd(start, _) => {
                let date = date.and_time(Utc::now().time()).unwrap();
                (start - date).num_milliseconds() < 0
            }
            Self::WholeDays(start, _) => (start - date).num_milliseconds() < 0,
        }
    }

    pub fn same_with_new_start_day(&self, new_date: Date<Utc>) -> Self {
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

    pub fn same_with_new_start_week(&self, new_date: Date<Utc>) -> Option<Self> {
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

    pub fn same_with_new_start_month(&self, new_date: Date<Utc>) -> Option<Self> {
        let period = self.clone();
        use chrono::Datelike;

        let (starting_month_day, ending_month_day) = match period {
            Self::StartEnd(start, end) => (start.date().day(), end.date().day()),
            Self::WholeDays(start, end) => (start.day(), end.day()),
        };

        match new_date.day() {
            value if value >= starting_month_day && value <= ending_month_day => {
                let day_difference = (value - starting_month_day) as i64;
                let new_date = new_date - Duration::days(day_difference);

                Some(self.same_with_new_start_day(new_date))
            }
            _ => None,
        }
    }

    pub fn same_with_new_start_year(&self, new_date: Date<Utc>) -> Option<Self> {
        let period = self.clone();
        use chrono::Datelike;

        let (
            (starting_month, starting_month_day),
            (ending_month, ending_month_day),
            day_difference,
        ) = match period {
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

        if new_date.day() < starting_month_day || new_date.day() > ending_month_day {
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

    pub fn get_date_time_start(&self) -> DateTime<Utc> {
        match self.clone() {
            Self::StartEnd(start, _) => start.clone(),
            Self::WholeDays(start, _) => start.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap(),
        }
    }
}

impl Default for EventPeriod {
    fn default() -> Self {
        Self::WholeDays(Utc::today(), Utc::today() + Duration::days(1))
    }
}
