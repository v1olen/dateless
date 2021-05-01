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
