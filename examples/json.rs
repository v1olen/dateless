fn main() {
    use chrono::{Duration, Utc};
    use dateless::prelude::*;

    let event = EventPartial::new(String::from("Date"))
        .with_period(EventPeriod::StartEnd(
            Utc::now(),
            Utc::now() + Duration::hours(1),
        ))
        .weekly()
        .complete();

    println!("{}", serde_json::to_string(&event).unwrap());
}
