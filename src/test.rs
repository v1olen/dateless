test!(one_day, [Duration::days(1), Duration::days(-1)]);

test!(
    two_days,
    whole_days,
    WholeDays,
    today,
    Duration::days(1),
    [Duration::days(2), Duration::days(-1)]
);

test!(
    one_hour,
    from_to,
    StartEnd,
    now,
    Duration::hours(1),
    [Duration::days(2), Duration::days(-1)]
);
