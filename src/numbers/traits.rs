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

pub trait AsNum<T> {
    fn as_num(&self) -> T {
        let x = self.try_as_num();
        if let Some(n) = x {
            n
        } else {
            panic!("oops")
        }
    }

    fn try_as_num(&self) -> Option<T>;
}

pub trait Sqrt<T> {
    fn sqrt(&self) -> T {
        let x = self.try_sqrt();
        if let Some(s) = x {
            s
        } else {
            panic!("oops")
        }
    }

    fn try_sqrt(&self) -> Option<T>;
}

impl Sqrt<Real> for Real {
    fn try_sqrt(&self) -> Option<Real> {
        if self >= &Real::zero() {
            let val = self.value().sqrt();
            Some(real!(val))
        } else {
            None
        }
    }
}

impl Sqrt<Complex> for Real {
    fn try_sqrt(&self) -> Option<Complex> {
        Some(if self >= &Real::zero() {
            let val = self.value().sqrt();
            complex!(val)
        } else {
            let val = (-self.value()).sqrt();
            complex!(val.i)
        })
    }
}

macro_rules! impl_as_num_self {
    ($($name:ty)*) => { $(impl_as_num_self!{ @impl $name })* };
    (@impl $name:ty) => {
        impl AsNum<$name> for $name {
            fn try_as_num(&self) -> Option<$name> { Some(*self) }
        }
    }
}

impl_as_num_self! { Natural Integer Rational Real Complex }

impl AsNum<Natural> for Integer {
    fn try_as_num(&self) -> Option<Natural> {
        Some(self.abs())
    }
}

impl AsNum<Integer> for Rational {
    fn try_as_num(&self) -> Option<Integer> {
        if self.numer % self.denom == integer!(0) {
            Some(self.numer / self.denom)
        } else {
            None
        }
    }
}

impl AsNum<Real> for Complex {
    fn try_as_num(&self) -> Option<Real> {
        if self.is_real() {
            Some(self.real)
        } else {
            None
        }
    }
}
