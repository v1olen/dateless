use chrono::{Duration, Utc};

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Serialize};

use super::Cyclicity;
use crate::event::period::Period;

#[derive(Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct AnnualCycle;

#[cfg_attr(feature = "serde_support", typetag::serde)]
impl Cyclicity for AnnualCycle {
    fn same_period_at(
        &self,
        same_period: Box<dyn Period>,
        at_date: chrono::Date<Utc>,
    ) -> Option<Box<dyn Period>> {
        use chrono::Datelike;

        let (starting_month, ending_month) = same_period.as_months();
        let (starting_month_day, ending_month_day) = same_period.as_days_of_month();
        let day_difference = at_date - same_period.with_new_month(at_date.month());

        if at_date.day() < starting_month_day || at_date.day() > ending_month_day {
            return None;
        }

        let day_difference = day_difference.num_days();

        match at_date.month() {
            value if value >= starting_month && value <= ending_month => {
                let at_date = at_date - Duration::days(day_difference);

                Some(same_period.with_new_start(at_date))
            }
            _ => None,
        }
    }
}
