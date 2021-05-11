#[cfg(test)]
#[macro_export]
macro_rules! test {
    ($name:ident, $ne_items:expr) => {
        #[test]
        fn $name() {
            use crate::{
                event::{PeriodDef, WholeDays},
                prelude::*,
            };

            use chrono::{Duration, Utc};

            let today = Utc::today();

            let event = EventPartial::new(String::from("Date"))
                .whole_day(today)
                .complete();

            helper!(
                event,
                PeriodDef(Box::new(WholeDays(today, today))),
                today,
                $ne_items
            );
        }
    };
    ($name:ident, $period:ident, $period_struct:ident, $today_or_now:ident, $dur:expr, $ne_items:expr) => {
        #[test]
        fn $name() {
            use crate::{event::PeriodDef, prelude::*};

            use chrono::{Duration, Utc};

            let moment = Utc::$today_or_now();

            let event = EventPartial::new(String::from("Date"))
                .$period(moment, moment + $dur)
                .complete();

            helper!(
                event,
                PeriodDef(Box::new(crate::event::$period_struct(
                    moment,
                    moment + $dur
                ))),
                moment,
                $ne_items
            );
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! helper {
    ($event:expr, $period:expr, $day:expr, $ne_items:expr) => {
        let event_uuid = $event.uuid.clone();

        let mut calendar = Calendar::new();
        calendar.add_event($event);

        let expected_occurrence = serde_json::to_string(&vec![EventOccurrence {
            origin: event_uuid,
            name: "Date".into(),
            description: None,
            period: $period,
        }])
        .unwrap();

        assert_eq!(
            serde_json::to_string(&calendar.day(Utc::today())).unwrap(),
            expected_occurrence,
        );

        for item in $ne_items.iter() {
            assert_ne!(
                serde_json::to_string(&calendar.day(Utc::today() + item.clone())).unwrap(),
                expected_occurrence,
            );
        }
    };
}

#[macro_export]
macro_rules! bind_partial_filler {
    ($name:ident, $field:ident, $type:ident) => {
        pub fn $name(self, $field: $type) -> Self {
            let $field = Some($field);

            Self { $field, ..self }
        }
    };
    ($name:ident, $field:ident) => {
        pub fn $name<T: ToString>(self, $field: T) -> Self {
            let $field = Some($field.to_string());

            Self { $field, ..self }
        }
    };
}

#[macro_export]
macro_rules! bind_partial_filler_boxed {
    ($name:ident, $field:ident, $type:ident) => {
        fn $name(self, $field: Box<dyn $type>) -> Self {
            let $field = Some($field);

            Self { $field, ..self }
        }
    };
}

#[macro_export]
macro_rules! bind_partial_filler_default {
    ($name:ident, $field:ident) => {
        pub fn $name<T: ToString>($field: T) -> Self {
            let $field = Some($field.to_string());

            Self {
                $field,
                ..Default::default()
            }
        }
    };
}

#[macro_export]
macro_rules! bind_partial_trait_filler {
    ($name:ident, $type:ident, $method:ident) => {
        pub fn $name(self) -> Self {
            self.$method(Box::new($type))
        }
    };
}
