// Copyright 2018 Vlad Yermakov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{Complex, Integer, Natural, Rational, Real};

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Natural(Natural),
    Integer(Integer),
    Rational(Rational),
    Real(Real),
    Complex(Complex),
}

impl Number {
    pub fn natural(s: String) -> Result<Number, <i128 as FromStr>::Err> {
        let nat = <i128>::from_str(s.as_str())?;
        Ok(Number::Natural(natural!(nat)))
    }

    pub fn rational(n: String, d: String) -> Result<Number, <i128 as FromStr>::Err> {
        let num = <i128>::from_str(n.as_str())?;
        let den = <i128>::from_str(d.as_str())?;
        Ok(Number::Rational(rational!(num / den)))
    }

    pub fn complex(s: String) -> Result<Number, <f64 as FromStr>::Err> {
        let com = <f64>::from_str(s.as_str())?;
        Ok(Number::Complex(complex!(com.i)))
    }

    pub fn real(s: String) -> Result<Number, <f64 as FromStr>::Err> {
        let real = <f64>::from_str(s.as_str())?;
        Ok(Number::Real(real!(real)))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Number::Natural(val) => val.fmt(f),
            Number::Integer(val) => val.fmt(f),
            Number::Rational(val) => val.fmt(f),
            Number::Real(val) => val.fmt(f),
            Number::Complex(val) => val.fmt(f),
        }
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        match self {
            Number::Natural(nat) => Number::Integer(-nat),
            Number::Integer(int) => Number::Integer(-int),
            Number::Rational(rat) => Number::Rational(-rat),
            Number::Real(rea) => Number::Real(-rea),
            Number::Complex(com) => Number::Complex(-com),
        }
    }
}

macro_rules! impl_ops_for_number {
    () => {impl_ops_for_number! { (Add; add; +) (Sub; sub; -) (Mul; mul; *) (Div; div; /) } };
    ($(($tr:ty; $name:ident; $op:tt))*) => { $(impl_ops_for_number! { @impl $tr; $name; $op } )* };
    (@impl $tr:ty; $name:ident; $op:tt) => {
        impl $tr for Number {
            type Output = Number;

            fn $name(self, other: Number) -> Number {
                match self {
                    Number::Natural(nat) => match other {
                        Number::Natural(oth) => Number::Natural(nat $op oth),
                        Number::Integer(int) => Number::Integer(nat $op int),
                        Number::Rational(rat) => Number::Rational(nat $op rat),
                        Number::Real(rea) => Number::Real(nat $op rea),
                        Number::Complex(com) => Number::Complex(nat $op com),
                    },
                    Number::Integer(int) => match other {
                        Number::Natural(nat) => Number::Integer(int $op nat),
                        Number::Integer(oth) => Number::Integer(int $op oth),
                        Number::Rational(rat) => Number::Rational(int $op rat),
                        Number::Real(rea) => Number::Real(int $op rea),
                        Number::Complex(com) => Number::Complex(int $op com),
                    },
                    Number::Rational(rat) => match other {
                        Number::Natural(nat) => Number::Rational(rat $op nat),
                        Number::Integer(int) => Number::Rational(rat $op int),
                        Number::Rational(oth) => Number::Rational(rat $op oth),
                        Number::Real(rea) => Number::Real(rat $op rea),
                        Number::Complex(com) => Number::Complex(rat $op com),
                    },
                    Number::Real(rea) => match other {
                        Number::Natural(nat) => Number::Real(rea $op nat),
                        Number::Integer(int) => Number::Real(rea $op int),
                        Number::Rational(rat) => Number::Real(rea $op rat),
                        Number::Real(oth) => Number::Real(rea $op oth),
                        Number::Complex(com) => Number::Complex(rea $op com),
                    },
                    Number::Complex(com) => match other {
                        Number::Natural(nat) => Number::Complex(com $op nat),
                        Number::Integer(int) => Number::Complex(com $op int),
                        Number::Rational(rat) => Number::Complex(com $op rat),
                        Number::Real(rea) => Number::Complex(com $op rea),
                        Number::Complex(oth) => Number::Complex(com $op oth),
                    },
                }
            }
        }
    }
}

impl_ops_for_number!{}

//macro_rules! product {
//    ($first:tt) => { product! { @product $first; $first } };
//    (@product [$($first:ident);*]; $second:tt ) => {{
//        let mut cnt = 0;
//        $(
//            cnt = cnt + 1;
//            product! { @first $first; cnt; $second }
//        )*
//    }};
//    (@first $first:ident; $cnt_first:expr; [$($second:ident);*] ) => {{
//        let mut cnt = 0;
//        $(
//            cnt = cnt + 1;
//            product! { @second $first; $cnt_first; $second; cnt }
//        )*
//    }};
//    (@second $first:ident; $cnt_first:expr; $second:ident; $cnt_second:expr) => {
//        if $cnt_first <= $cnt_second {
//            ($first, $second)
//        }
//    }
//}

//#[test]
//fn test_product() {
//    let a = 1;
//    let b = 2;
//    let c = 3;
//    let d = 4;
//    let e = 5;
//    product!([a; b; c; d; e]);
//}
