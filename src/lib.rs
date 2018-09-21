extern crate decimal;

pub use decimal::d128;
pub type Status = decimal::Status;
use std::ops;

pub trait ExactCurrency:
    ops::Add
    + ops::Mul<d128>
    + ops::Mul<i32>
    + ops::Mul<u32>
    + ops::Mul<u64>
    + ops::Mul<i64>
where
    Self: std::marker::Sized,
{
    fn new() -> Self;
    fn amount(&self) -> &d128;
    fn set_amount(&mut self, amount: d128);
    fn abbreviation(&self) -> Option<&'static str>;
    fn symbol(&self) -> Option<char>;
}

macro_rules! define_currency {
    ($currency: ident, $abbreviation: expr,
     $symbol: expr) => {
        pub struct $currency {
            amount: d128,
        }

        impl ExactCurrency for $currency {
            fn new() -> $currency {
                $currency {
                    amount: d128::zero(),
                }
            }

            fn amount(&self) -> &d128 {
                &self.amount
            }

            fn set_amount(&mut self, amount: d128) {
                self.amount = amount;
            }

            fn abbreviation(&self) -> Option<&'static str> {
                Some($abbreviation)
            }

            fn symbol(&self) -> Option<char> {
                Some($symbol)
            }
        }

        impl ops::Add for $currency {
            type Output = Result<Self, (Status, Self, Self)>;
            fn add(self, rhs: Self) -> Result<Self, (Status, Self, Self)> {
                d128_clear_flags();
                let result = Self {
                    amount: self.amount + rhs.amount,
                };
                d128_wrap_result(result, self, rhs)
            }
        }

        impl ops::Sub for $currency {
            type Output = Result<Self, (Status, Self, Self)>;
            fn sub(self, rhs: Self) -> Result<Self, (Status, Self, Self)> {
                d128_clear_flags();
                let result = Self { 
                    amount: self.amount - rhs.amount,
                };
                d128_wrap_result(result, self, rhs)
            }
        }

        impl_mul!($currency, d128);
        impl_mul!($currency, i32);
        impl_mul!($currency, u32);
        impl_mul!($currency, i64);
        impl_mul!($currency, u64);
    };
}

macro_rules! impl_mul {
    ($currency: ident, $type: ident) => {
        impl ops::Mul<$type> for $currency {
            type Output=Result<Self, (Status, Self, $type)>;
            fn mul(self, rhs: $type) -> Result<Self, (Status, Self, $type)> {
                d128_clear_flags();
                let result = Self {
                    amount: self.amount * d128::from(rhs),
                };
                d128_wrap_result(result, self, rhs)
            }
        }
    };
}

define_currency!(AUD, "AUD", '$');
define_currency!(USD, "USD", '$');
define_currency!(GBP, "GBP", '£');

fn d128_clear_flags() {
    d128::set_status(Status::empty())
}

fn d128_wrap_result<T: ExactCurrency, U>(result: T, lhs: T, rhs: U) -> Result<T, (Status, T, U)> {
    match d128::get_status() == Status::empty() {
        true => Ok(result),
        false => {
            d128_clear_flags();
            Err((d128::get_status(), lhs, rhs))
        }
    }
}