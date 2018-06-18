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

use super::Real;

use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Complex {
    pub(crate) real: Real,
    pub(crate) imag: Real,
}

impl Complex {
    pub fn new<U, V>(real: U, imag: V) -> Complex
    where
        U: Into<Real>,
        V: Into<Real>,
    {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }

    pub fn abs(&self) -> Real {
        self.real * self.real + self.imag * self.imag
    }

    pub fn inv(&self) -> Complex {
        Complex::new(Real::new(1.), Real::new(0.)) / *self
    }

    pub fn conj(&self) -> Complex {
        Complex::new(self.real, -self.imag)
    }

    pub fn is_real(&self) -> bool {
        self.imag == Real::zero()
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.real.fmt(f)?;
        if self.imag >= Real::zero() {
            " + ".fmt(f)?;
            self.imag.fmt(f)?;
        } else {
            " - ".fmt(f)?;
            (-self.imag).fmt(f)?;
        }
        "i".fmt(f)
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Complex) -> Option<Ordering> {
        if self.real == other.real {
            return self.imag.partial_cmp(&other.imag);
        }
        if self.imag == other.imag {
            return self.real.partial_cmp(&other.real);
        }
        return None;
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex::new(-self.real, -self.imag)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + other.imag * self.real,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        Complex {
            real: (self * other.conj()).real / (other.abs()),
            imag: (self * other.conj()).imag / (other.abs()),
        }
    }
}

#[macro_export]
macro_rules! complex {
    ($a:tt + $b:tt.i) => {
        $crate::numbers::Complex::new($a as f64, $b as f64)
    };
    (- $a:tt + $b:tt.i) => {
        $crate::numbers::Complex::new(-$a as f64, $b as f64)
    };
    ($a:tt - $b:tt.i) => {
        $crate::numbers::Complex::new($a as f64, -$b as f64)
    };
    (- $a:tt - $b:tt.i) => {
        $crate::numbers::Complex::new(-$a as f64, -$b as f64)
    };
    ($a:tt) => {
        $crate::numbers::Complex::new($a as f64, 0 as f64)
    };
    (- $a:tt) => {
        $crate::numbers::Complex::new(-$a as f64, 0 as f64)
    };
    ($b:tt.i) => {
        $crate::numbers::Complex::new(0 as f64, $b as f64)
    };
    (- $b:tt.i) => {
        $crate::numbers::Complex::new(0 as f64, -$b as f64)
    };
}

impl_default! { Complex, complex!(0) }
