#[macro_export]
macro_rules! make_error {
    ($name:ident, $typ:ident, $enum_value:ident) => {
        pub mod $name {
            use thiserror::Error;

            #[derive(Debug, Clone, Error)]
            pub enum Error {
                #[error("must be equal to {0}")]
                Const($typ),
                #[error("must be less than {0}")]
                Lt($typ),
                #[error("must be less than or equal to {0}")]
                Lte($typ),
                #[error("must be greater than {0}")]
                Gt($typ),
                #[error("must be greater than or equal to {0:?}")]
                Gte($typ),
                #[error("must be in range {0}{1}, {2}{3}")]
                InRange(String, $typ, $typ, String),
                #[error("must not be in range {0}{1}, {2}{3}")]
                NotInRange(String, $typ, $typ, String),
                #[error("must be in {0:?}")]
                In(Vec<$typ>),
                #[error("must not be in {0:?}")]
                NotIn(Vec<$typ>),
            }

            impl Error {
                pub fn in_range(
                    start_inclusive: bool,
                    start: $typ,
                    end: $typ,
                    end_inclusive: bool,
                ) -> Self {
                    Self::InRange(
                        if start_inclusive { "[" } else { "(" }.to_string(),
                        start,
                        end,
                        if end_inclusive { "]" } else { ")" }.to_string(),
                    )
                }
                pub fn not_in_range(
                    start_inclusive: bool,
                    start: $typ,
                    end: $typ,
                    end_inclusive: bool,
                ) -> Self {
                    Self::NotInRange(
                        if start_inclusive { "[" } else { "(" }.to_string(),
                        start,
                        end,
                        if end_inclusive { "]" } else { ")" }.to_string(),
                    )
                }
            }
            impl From<Error> for crate::errors::Error {
                fn from(value: Error) -> Self {
                    Self::$enum_value(value)
                }
            }
        }
    };
}
