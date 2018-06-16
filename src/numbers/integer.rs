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

use super::Natural;

use std::convert::From;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Integer(i128); // TODO: --//--

impl Integer {
    pub fn new<T: Into<i128>>(int: T) -> Integer {
        Integer(int.into())
    }

    pub fn abs(&self) -> Natural {
        if self.0 >= 0 {
            Natural::new(self.0)
        } else {
            Natural::new(-self.0)
        }
    }

    pub fn value(&self) -> i128 {
        self.0
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Into<i128>> From<T> for Integer {
    fn from(some: T) -> Integer {
        Integer::new(some.into())
    }
}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        Integer::new(-self.0)
    }
}

#[macro_export]
macro_rules! integer {
    ($int:tt) => {
        $crate::numbers::Integer::new($int as i128)
    };
    (- $int:tt) => {
        $crate::numbers::Integer::new(-$int as i128)
    };
}

impl_std_ops_for_tuple_struct! { Integer: @all }
impl_std_ops_for_tuple_struct! { Integer: Rem(rem, %) }
impl_default! { Integer, integer!(0) }
