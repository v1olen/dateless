use chrono::{Date, NaiveDate, Utc};
use serde::{de::Deserializer, ser::Serializer, Deserialize};
use std::str::FromStr;

pub fn from_string_into_date<'de, D>(d: D) -> Result<Date<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(d) {
        Ok(value) => Ok({
            let date = NaiveDate::from_str(&value);
            Date::from_utc(date.unwrap(), Utc)
        }),
        Err(error) => Err(error),
    }
}

pub fn from_date_into_string<S>(date: &Date<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&date.naive_utc().format("%Y-%m-%d").to_string())
}
