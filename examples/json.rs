fn main() {
    use chrono::{Duration, Utc};
    use dateless::prelude::*;

    let event = EventPartial::new(String::from("Date"))
        .from_to(Utc::now(), Utc::now() + Duration::hours(1))
        .daily()
        .complete();

    println!("{}", serde_json::to_string(&event).unwrap());
}
