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

use numbers::Integer;

use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Natural(i128); // TODO: change to long math

impl Natural {
    pub fn new<T: Into<i128>>(nat: T) -> Natural {
        let nat = nat.into();
        if nat < 0 {
            panic!{}
        }
        Natural(nat)
    }

    pub fn zero() -> Natural {
        Natural(0)
    }

    pub fn value(&self) -> i128 {
        self.0
    }
}

impl Display for Natural {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub trait AsNat {
    fn as_nat(&self) -> Natural;
}

impl AsNat for Natural {
    fn as_nat(&self) -> Natural {
        *self
    }
}

impl Neg for Natural {
    type Output = Integer;

    fn neg(self) -> Integer {
        let self_: Integer = self.into();
        -self_
    }
}

impl_std_ops_for_tuple_struct! { Natural: @all }
impl_std_ops_for_tuple_struct! { Natural: Rem(rem, %) }

#[macro_export]
macro_rules! natural {
    ($nat:tt) => {
        $crate::numbers::Natural::new($nat)
    };
}

impl_default! { Natural, natural!(0) }
