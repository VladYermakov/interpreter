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

use super::Integer;
use utils::gcd;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Rational {
    pub(crate) numer: Integer,
    pub(crate) denom: Integer,
}

impl Rational {
    pub fn new<U, V>(numer: U, denom: V) -> Rational
    where
        U: Into<Integer>,
        V: Into<Integer>,
    {
        let mut self_ = Rational {
            numer: numer.into(),
            denom: denom.into(),
        };
        self_.norm();
        self_
    }

    pub fn norm(&mut self) {
        let gcd = gcd(self.numer, self.denom);

        self.numer = self.numer / gcd;
        self.denom = self.denom / gcd;

        if self.denom < integer!(0) {
            self.numer = self.numer * integer!(-1);
            self.denom = self.denom * integer!(-1);
        }
    }

    pub fn inv(&self) -> Rational {
        Rational::new(self.denom, self.numer)
    }

    pub fn value(&self) -> f64 {
        (self.numer.value() as f64) / (self.denom.value() as f64)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.numer.fmt(f)?;
        " / ".fmt(f)?;
        self.denom.fmt(f)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Rational) -> Ordering {
        (self.numer * other.denom).cmp(&(self.denom * other.numer))
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        Rational::new(
            self.numer * other.denom + self.denom * other.numer,
            self.denom * other.denom,
        )
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        Rational::new(
            self.numer * other.denom - self.denom * other.numer,
            self.denom * other.denom,
        )
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.numer * other.numer, self.denom * other.denom)
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        self * other.inv()
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational::new(-self.numer, self.denom)
    }
}

#[macro_export]
macro_rules! rational {
    ($a:tt / $b:tt) => {
        $crate::numbers::Rational::new($a, $b)
    };
    (- $a:tt / $b:tt) => {
        $crate::numbers::Rational::new(-$a, $b)
    };
    ($a:tt) => {
        $crate::numbers::Rational::new($a, 1)
    };
    (- $a:tt) => {
        $crate::numbers::Rational::new(-$a, 1)
    };
}

impl_default! { Rational, rational!(0)}
