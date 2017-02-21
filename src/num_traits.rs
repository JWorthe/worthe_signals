use std;

pub trait Trig {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

macro_rules! impl_float_trig {
    ($t: ty) => {
        impl Trig for $t {
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn tan(self) -> Self {
                self.tan()
            }
            fn asin(self) -> Self {
                self.asin()
            }
            fn acos(self) -> Self {
                self.acos()
            }
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
        }
    }
}

impl_float_trig!(f32);
impl_float_trig!(f64);


pub trait Pow {
    fn pow(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
}

macro_rules! impl_float_pow {
    ($t: ty) => {
        impl Pow for $t {
            fn pow(self, n: i32) -> Self {
                self.powi(n)
            }
            fn sqrt(self) -> Self {
                self.sqrt()
            }
        }
    }
}

macro_rules! impl_int_pow {
    ($t: ty) => {
        impl Pow for $t {
            fn pow(self, n: i32) -> Self {
                if n > 0 {
                    self.pow(n as u32)
                } else {
                    (self as f64).powi(n) as Self
                }
            }
            fn sqrt(self) -> Self {
                (self as f64).sqrt() as Self
            }
        }
    }
}

impl_float_pow!(f32);
impl_float_pow!(f64);
impl_int_pow!(i8);
impl_int_pow!(i16);
impl_int_pow!(i32);
impl_int_pow!(i64);
impl_int_pow!(u8);
impl_int_pow!(u16);
impl_int_pow!(u32);
impl_int_pow!(u64);


use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

pub trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Rem<Output=Self> where Self: std::marker::Sized {}
impl<T> ArithmeticOps for T where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> + Rem<Output=T> {}

pub trait SignedArithmeticOps: ArithmeticOps + Neg<Output=Self> where Self: std::marker::Sized {}
impl<T> SignedArithmeticOps for T where T: ArithmeticOps + Neg<Output=T> {}


pub trait FractionOps {
    fn recip(self) -> Self;
    fn pi() -> Self;
    fn two_pi() -> Self;
    fn half_pi() -> Self;
    fn zero() -> Self;
}

macro_rules! impl_fraction_float {
    ($t: ty, $pi: expr) => {
        impl FractionOps for $t {
            fn recip(self) -> Self {
                self.recip()
            }
            fn pi() -> Self {
                $pi
            }
            fn two_pi() -> Self {
                2.0 * $pi
            }
            fn half_pi() -> Self {
                $pi / 2.0
            }
            fn zero() -> Self {
                0.0
            }
        }
    }
}

impl_fraction_float!(f32, std::f32::consts::PI);
impl_fraction_float!(f64, std::f64::consts::PI);
