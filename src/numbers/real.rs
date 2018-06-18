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

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialOrd, Debug, Copy, Clone)]
pub struct Real(f64); // TODO: --//--

const EPS: Real = Real(1e-14);

impl Real {
    pub fn new<T: Into<f64>>(real: T) -> Real {
        Real(real.into())
    }

    pub fn zero() -> Real {
        Real::new(0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn abs(&self) -> Real {
        Real::new(self.0.abs())
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq for Real {
    fn eq(&self, other: &Real) -> bool {
        (*self - *other).abs() < EPS
    }
}

impl<T: Into<f64>> From<T> for Real {
    fn from(some: T) -> Real {
        Real::new(some.into())
    }
}

impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Real {
        Real::new(-self.0)
    }
}

impl_std_ops_for_tuple_struct! { Real: @all }

#[macro_export]
macro_rules! real {
    ($a:expr) => {
        $crate::numbers::Real::new($a as f64)
    };
}

impl_default! { Real, real!(0) }
