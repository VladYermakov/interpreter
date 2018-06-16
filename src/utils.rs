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

use numbers::{AsNum, Natural};

pub fn gcd<U, V>(a: U, b: V) -> Natural
where
    U: AsNum<Natural>,
    V: AsNum<Natural>,
{
    return gcd_nat(a.as_num(), b.as_num());
}

pub fn gcd_nat(a: Natural, b: Natural) -> Natural {
    let mut a = a;
    let mut b = b;
    while a > Natural::zero() && b > Natural::zero() {
        if a > b {
            a = a % b
        } else {
            b = b % a
        }
    }

    return a + b;
}
