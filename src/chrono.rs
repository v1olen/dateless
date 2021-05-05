use chrono::{Date, DateTime, Utc};

#[cfg(feature = "serde_support")]
use chrono::serde::ts_seconds;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde_support")]
use crate::serde::{from_date_into_string, from_string_into_date};

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

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct DateTimeDef(
    #[cfg_attr(feature = "serde_support", serde(with = "ts_seconds"))] pub DateTime<Utc>,
);
