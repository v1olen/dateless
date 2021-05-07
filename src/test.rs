use crate::{
    event::{PeriodDef, StartEnd, WholeDays},
    prelude::*,
};

use chrono::{DateTime, Duration, Utc};

#[test]
fn one_day_and_one_time_date() {
    let mut calendar = Calendar::new();

    let today = Utc::today();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    let event = EventPartial::new(String::from("Date"))
        .whole_days(today, today)
        .complete();

    calendar.add_event(event);

    let expected_occurrence = serde_json::to_string(&vec![EventOccurrence {
        name: "Date".into(),
        description: None,
        period: PeriodDef(Box::new(WholeDays(today, today))),
    }])
    .unwrap();

    assert_ne!(
        serde_json::to_string(&calendar.day(yesterday)).unwrap(),
        expected_occurrence,
    );
    assert_eq!(
        serde_json::to_string(&calendar.day(today)).unwrap(),
        expected_occurrence,
    );
    assert_ne!(
        serde_json::to_string(&calendar.day(tomorrow)).unwrap(),
        expected_occurrence,
    );
}

#[test]
fn one_hour_and_one_time_date() {
    let mut calendar = Calendar::new();

    let today = Utc::today();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    let (date_start, date_end) = (Utc::now(), Utc::now() + Duration::hours(1));
    let event = EventPartial::new(String::from("Date"))
        .from_to(date_start, date_end)
        .complete();

    calendar.add_event(event);

    let expected_occurrence = serde_json::to_string(&vec![EventOccurrence {
        name: "Date".into(),
        description: None,
        period: PeriodDef(Box::new(StartEnd(date_start, date_end))),
    }])
    .unwrap();

    assert_ne!(
        serde_json::to_string(&calendar.day(yesterday)).unwrap(),
        expected_occurrence,
    );
    assert_eq!(
        serde_json::to_string(&calendar.day(today)).unwrap(),
        expected_occurrence,
    );
    assert_ne!(
        serde_json::to_string(&calendar.day(tomorrow)).unwrap(),
        expected_occurrence,
    );
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
        .from_to(date_start, date_end)
        .complete();

    calendar.add_event(event);

    let expected_occurrence = serde_json::to_string(&vec![EventOccurrence {
        name: "Date".into(),
        description: None,
        period: PeriodDef(Box::new(StartEnd(date_start, date_end))),
    }])
    .unwrap();

    assert_ne!(
        serde_json::to_string(&calendar.day(two_days_later)).unwrap(),
        expected_occurrence,
    );
    assert_eq!(
        serde_json::to_string(&calendar.day(today)).unwrap(),
        expected_occurrence,
    );
    assert_eq!(
        serde_json::to_string(&calendar.day(tomorrow)).unwrap(),
        expected_occurrence,
    );
    assert_ne!(
        serde_json::to_string(&calendar.day(yesterday)).unwrap(),
        expected_occurrence,
    );
}
