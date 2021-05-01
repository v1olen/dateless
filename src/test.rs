#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use chrono::{Duration, Utc};

    #[test]
    fn one_day_and_one_time_date() {
        let calendar = Calendar::new();

        let today = Utc::today();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        let event = EventPartial::new(String::from("Date"))
            .with_period(EventPeriod::WholeDays(today, today))
            .complete();

        let calendar = calendar.add_event(event);

        let expected_occurrence = vec![EventOccurrence {
            name: "Date".into(),
            description: None,
            period: EventPeriod::WholeDays(today, today),
        }];

        assert_ne!(calendar.day(yesterday), expected_occurrence);
        assert_eq!(calendar.day(today), expected_occurrence);
        assert_ne!(calendar.day(tomorrow), expected_occurrence);
    }

    #[test]
    fn one_hour_and_one_time_date() {
        let calendar = Calendar::new();

        let today = Utc::today();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        let (date_start, date_end) = (Utc::now(), Utc::now() + Duration::hours(1));
        let event = EventPartial::new(String::from("Date"))
            .with_period(EventPeriod::StartEnd(date_start, date_end))
            .complete();

        let calendar = calendar.add_event(event);

        let expected_occurrence = vec![EventOccurrence {
            name: "Date".into(),
            description: None,
            period: EventPeriod::StartEnd(date_start, date_end),
        }];

        assert_ne!(calendar.day(yesterday), expected_occurrence);
        assert_eq!(calendar.day(today), expected_occurrence);
        assert_ne!(calendar.day(tomorrow), expected_occurrence);
    }
}
