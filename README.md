<div align="center">
    <img src="https://cdn.v-sn.io/dateless-logo" alt="" width="192" height="192">
    <br>
    <br>
</div>

<div align="center"><h3>Dateless</h3></div>

<div align="center">
  <em color="#aaa"></em>
  <br>
  <a href="https://crates.io/crates/dateless">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/dateless">
  </a>
  <a href="https://gitlab.com/v1olen/dateless/-/commits/master">
    <img alt="pipeline status" src="https://gitlab.com/v1olen/dateless/badges/master/pipeline.svg" />
  </a>
  <a href="https://gitlab.com/v1olen/dateless/-/blob/master/LICENSE">
    <img alt="license" src="https://img.shields.io/crates/l/dateless">
  </a>
  <br>
  <br>
</div>

# Dateless

Dateless is an events & calendar library for Rust.

## Usage

```rust
use dateless::prelude::*;
use chrono::{Utc, Duration};

fn main() {
    let mut calendar = Calendar::new();

    let event = EventPartial::new(String::from("Anne's birthday"))
        .whole_days(Utc::today(), Utc::today())
        .weekly()
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

## Contribution

Soon
