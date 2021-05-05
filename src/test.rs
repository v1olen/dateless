use crate::prelude::*;
use chrono::{DateTime, Duration, Utc};

#[test]
fn one_day_and_one_time_date() {
    let mut calendar = Calendar::new();

    let today = Utc::today();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    let event = EventPartial::new(String::from("Date"))
        .with_period(EventPeriod::WholeDays(today, today))
        .complete();

    calendar.add_event(event);

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
    let mut calendar = Calendar::new();

    let today = Utc::today();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    let (date_start, date_end) = (Utc::now(), Utc::now() + Duration::hours(1));
    let event = EventPartial::new(String::from("Date"))
        .with_period(EventPeriod::StartEnd(date_start, date_end))
        .complete();

    calendar.add_event(event);

    let expected_occurrence = vec![EventOccurrence {
        name: "Date".into(),
        description: None,
        period: EventPeriod::StartEnd(date_start, date_end),
    }];

    assert_ne!(calendar.day(yesterday), expected_occurrence);
    assert_eq!(calendar.day(today), expected_occurrence);
    assert_ne!(calendar.day(tomorrow), expected_occurrence);
}

#[test]
fn subtract_datetime_from_date() {
    let mut calendar = Calendar::new();

    let today = Utc::today();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);
    let two_days_later = today + Duration::days(2);

    let date_start = DateTime::from_utc(today.naive_utc().and_hms(22, 59, 59), Utc);
    let date_end = DateTime::from_utc(two_days_later.naive_utc().and_hms(0, 0, 0), Utc);

    let event = EventPartial::new(String::from("Date"))
        .with_period(EventPeriod::StartEnd(date_start, date_end))
        .complete();

    calendar.add_event(event);

    let expected_occurrence = vec![EventOccurrence {
        name: "Date".into(),
        description: None,
        period: EventPeriod::StartEnd(date_start, date_end),
    }];

    assert_ne!(calendar.day(two_days_later), expected_occurrence);
    assert_eq!(calendar.day(today), expected_occurrence);
    assert_eq!(calendar.day(tomorrow), expected_occurrence);
    assert_ne!(calendar.day(yesterday), expected_occurrence);
}
