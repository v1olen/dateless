use chrono::{Duration, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Cyclicity;
use crate::event::period::Period;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct MonthlyCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Cyclicity for MonthlyCycle {
    fn same_period_at(
        &self,
        same_period: Box<dyn Period>,
        at_date: chrono::Date<Utc>,
    ) -> Option<Box<dyn Period>> {
        use chrono::Datelike;

        let (starting_month_day, ending_month_day) = same_period.as_days_of_month();

        match at_date.day() {
            value if value >= starting_month_day && value <= ending_month_day => {
                let day_difference = (value - starting_month_day) as i64;
                let at_date = at_date - Duration::days(day_difference);

                Some(same_period.with_new_start(at_date))
            }
            _ => None,
        }
    }
}
