#![doc(html_logo_url = "https://cdn.v-sn.io/dateless-logo")]

/*!

# Dateless

Dateless is an events & calendar library for Rust.

## Usage

First, add `dateless` as a dependency in your project's Cargo.toml:

```toml
[dependencies]
dateless = "0.2.3"
```

And then, you can start with creating a calendar:

```rust
use dateless::prelude::*;

fn main() {
    let mut calendar = Calendar::new();
}
```

Now, let's create an `Event` and assign it to the newly created `Calendar` instance. It can be done simply with `EventPartial`:

```rust
use dateless::prelude::*;
use chrono::Utc;

fn main() {
    let mut calendar = Calendar::new();

    let event = EventPartial::new(String::from("Anne's birthday"))
        .whole_days(Utc::today(), Utc::today())
        .daily()
        .complete();

    calendar.add_event(event);
}
```

Above, we created a new `Event` called "Anne's birthday" lasting all day today and set it to be recurring every week. In the last line we assigned the event to `calendar`.

Finally, we can check a specific day for occurrences of events.


```rust
use dateless::prelude::*;
use chrono::{Utc, Duration};

fn main() {
    let mut calendar = Calendar::new();

    let event = EventPartial::new(String::from("Anne's birthday"))
        .whole_days(Utc::today(), Utc::today())
        .daily()
        .complete();

    calendar.add_event(event);

    let seven_days_later = Utc::today() + Duration::days(7);

    println!("{:#?}", calendar.day(seven_days_later));
}
```

It prints to `stdout`:

```json
[
    EventOccurrence {
        name: "Anne's birthday",
        description: None,
        period: WholeDays(
            2021-05-08Z,
            2021-05-08Z,
        ),
    },
]
```

*/

#[macro_use]
mod codegen;

mod calendar;
mod chrono;
mod event;
pub mod prelude;

#[cfg(test)]
mod test;

#[cfg(feature = "serde_support")]
mod serde;

pub use calendar::Calendar;
pub use event::{Event, EventCyclicity, EventOccurrence, EventPartial, EventPeriod};
