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

#[macro_use]
mod macros;
#[macro_use]
mod complex;
#[macro_use]
mod integer;
#[macro_use]
mod rational;
#[macro_use]
mod real;
#[macro_use]
mod natural;
mod number;

mod traits;

mod cross_types;

#[cfg(test)]
mod tests;

pub use self::complex::Complex;
pub use self::integer::Integer;
pub use self::natural::Natural;
pub use self::number::Number;
pub use self::rational::Rational;
pub use self::real::Real;
pub use self::traits::{AsNum, Sqrt};
