use chrono::{Duration, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Cyclicity;
use crate::event::period::Period;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct WeeklyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Cyclicity for WeeklyCycle {
    fn same_period_at(
        &self,
        same_period: Box<dyn Period>,
        at_date: chrono::Date<Utc>,
    ) -> Option<Box<dyn Period>> {
        let (starting_weekday, ending_weekday) = same_period.as_weekdays();
        use chrono::Datelike;

        match at_date.weekday().number_from_monday() {
            value if value >= starting_weekday && value <= ending_weekday => {
                let day_difference = (value - starting_weekday) as i64;
                let at_date = at_date - Duration::days(day_difference);

                Some(same_period.with_new_start(at_date))
            }
            _ => None,
        }
    }
}
