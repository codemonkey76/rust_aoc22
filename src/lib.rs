use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result as DisplayResult};

pub mod day01;
pub mod day02;
pub mod day03;

pub enum Part {
    One,
    Two
}

macro_rules! impl_output_from {
    ( $( ($e:tt, $t:ty) ),* ) => {
        #[derive(Debug, Eq)]
        pub enum Output {
            $( $e($t), )*
        }

        $(
            impl From<$t> for Output {
                fn from(value: $t) -> Self {
                    Output::$e(value)
                }
            }
            )*
    };
}

impl_output_from! {
    (U8,    u8),
    (U16,   u16),
    (U32,   u32),
    (U64,   u64),
    (U128,  u128),
    (I8,    i8),
    (I16,   i16),
    (I32,   i32),
    (I64,   i64),
    (I128,  i128),
    (String, String)
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Output::U8(v) => write!(f, "{v}"),
            Output::U16(v) => write!(f, "{v}"),
            Output::U32(v) => write!(f, "{v}"),
            Output::U64(v) => write!(f, "{v}"),
            Output::U128(v) => write!(f, "{v}"),
            Output::I8(v) => write!(f, "{v}"),
            Output::I16(v) => write!(f, "{v}"),
            Output::I32(v) => write!(f, "{v}"),
            Output::I64(v) => write!(f, "{v}"),
            Output::I128(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
        }
    }
}

impl<T: Display> PartialEq<T> for Output {
    fn eq(&self, other: &T) -> bool {
        *self.to_string() == other.to_string()
    }
}
