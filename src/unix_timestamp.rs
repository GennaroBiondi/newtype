use chrono::{TimeZone, Utc};
use std::ops::{Add, Div, Mul, Sub};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct UnixTimestamp(pub i64);

macro_rules! impl_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for UnixTimestamp {
            type Output = UnixTimestamp;
            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0 $op rhs.0)
            }
        }
        impl $trait for &UnixTimestamp {
            type Output = UnixTimestamp;
            fn $method(self, rhs: Self) -> Self::Output {
                UnixTimestamp(self.0 $op rhs.0)
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

impl UnixTimestamp {
    pub fn now() -> Self {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        UnixTimestamp(secs)
    }

    pub fn prettify(&self) -> String {
        let dt = Utc.timestamp_opt(self.0, 0).single().unwrap();
        dt.to_rfc2822()
    }

    pub fn from_ago(&self) -> UnixTimestamp {
        &Self::now() - self
    }

    pub fn passed_from(&self, from: &UnixTimestamp) -> bool {
        self.0 <= from.0
    }
}
