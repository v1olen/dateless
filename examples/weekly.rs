use chrono::{Duration, Utc};
use dateless::prelude::*;

fn main() {
    let mut calendar = Calendar::new();

    let event = EventPartial::new(String::from("Anne's birthday"))
        .whole_day(Utc::today())
        .weekly()
        .complete();

    calendar.add_event(event);

    let seven_days_later = Utc::today() + Duration::days(7);

    println!("{:#?}", calendar.day(seven_days_later));
}
