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

#[macro_export]
macro_rules! impl_std_ops_for_tuple_struct {
    ($typename:ident : @all) => {
        impl_std_ops_for_tuple_struct! { $typename: Add(add, +), Sub(sub, -), Mul(mul, *), Div(div, /) }
    };
    ($typename:ident: $($trait_:ident($method:ident, $op:tt)),*) => {
        $(
            impl_std_ops_for_tuple_struct! { @impl $typename: $trait_($method, $op) }
        )*
    };
    (@impl $typename:ident: $trait_:ident($method:ident, $op:tt)) => {
        impl $trait_ for $typename {
            type Output = $typename;

            fn $method(self, other: $typename) -> $typename {
                $typename::new(self.value() $op other.value())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_default {
    ($t:ident, $v:expr) => {
        impl Default for $t {
            fn default() -> $t {
                $v
            }
        }
    };
}
