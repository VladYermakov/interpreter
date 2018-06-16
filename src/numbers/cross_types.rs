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

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use std::convert::Into;

macro_rules! impl_cross_type_ops {
    ($(($lhs:ty; $rhs:ty))*) => {
        $(
            impl_cross_type_ops! {
                @impl $lhs; @for $rhs; @ret $rhs;
                    @ops ((Add; add; +); (Sub; sub; -); (Mul; mul; *); (Div; div; /))
            }
            impl_cross_type_ops! {
                @impl $rhs; @for $lhs; @ret $rhs;
                    @ops ((Add; add; +); (Sub; sub; -); (Mul; mul; *); (Div; div; /))
            }
        )*
    };
    (@impl $rhs:ty; @for $lhs:ty; @ret $ths:ty; @ops ($(($cls:ident; $meth:ident; $op:tt));*)) => {
        $(
            impl_cross_type_ops! { @impl $rhs; @for $lhs; @ret $ths; @op $cls; $meth; $op }
        )*
    };
    (@impl $rhs:ty; @for $lhs:ty; @ret $ths:ty; @op $cls:ident; $meth:ident; $op:tt) => {
        impl $cls<$rhs> for $lhs {
            type Output = $ths;

            fn $meth(self, other: $rhs) -> $ths {
                let a: $ths = self.into();
                let b: $ths = other.into();

                a $op b
            }
        }
    }
}

macro_rules! impl_cross_type_assign_ops {
    ($(($rhs:ty; $lhs:ty))*) => {
        $(
            impl_cross_type_assign_ops! {
                @impl $rhs; @for $lhs;
                    @ops ((AddAssign; add_assign; +); (SubAssign; sub_assign; -);
                          (MulAssign; mul_assign; *); (DivAssign; div_assign; /))
            }
        )*
    };
    (@impl $rhs:ty; @for $lhs:ty; @ops ($(($cls:ident; $meth:ident; $op:tt));*)) => {
        $(
            impl_cross_type_assign_ops! { @impl $rhs; @for $lhs; @op $cls; $meth; $op }
        )*
    };
    (@impl $rhs:ty; @for $lhs:ty; @op $cls:ident; $meth:ident; $op:tt) => {
        impl $cls<$rhs> for $lhs {
            fn $meth(&mut self, other: $rhs) {
                let other: $lhs = other.into();

                *self = *self $op other;
            }
        }
    }
}

macro_rules! impl_cross_types_into {
    ($(($rhs:ty; $lhs:ty; $mcr:ident))*) => {
        $(
            impl_cross_types_into! { @impl $rhs; @for $lhs; @macro $mcr }
        )*
    };
    (@impl $rhs:ty; @for $lhs:ty; @macro $mcr:ident) => {
        impl Into<$lhs> for $rhs {
            fn into(self) -> $lhs {
                let val = self.value();
                $mcr!(val)
            }
        }
    }
}

macro_rules! impl_type_assign_ops {
    ($($name:ty)*) => { $( impl_cross_type_assign_ops! { ($name; $name) } )*};
}

macro_rules! impl_ops {
    ($(($rhs:ty; $lhs:ty; $mcr:ident))*) => {
        impl_cross_type_ops! { $(($rhs; $lhs))* }
        impl_cross_type_assign_ops! { $(($rhs; $lhs))* }
        impl_cross_types_into! { $(($rhs; $lhs; $mcr))* }
    };
}

impl_ops! { (Natural; Integer; integer) (Natural; Rational; rational) (Natural; Real; real) }
impl_ops! { (Natural; Complex; complex) (Integer; Rational; rational) (Integer; Real; real) }
impl_ops! { (Integer; Complex; complex) (Rational; Real; real) (Rational; Complex; complex) }
impl_ops! { (Real; Complex; complex) }

impl_cross_type_ops! { @impl Integer; @for Natural; @ret Integer; @op Rem; rem; % }
impl_cross_type_ops! { @impl Natural; @for Integer; @ret Integer; @op Rem; rem; % }

impl_cross_type_assign_ops! { @impl Natural; @for Integer; @op RemAssign; rem_assign; % }

impl_type_assign_ops! { Natural Integer Rational Real Complex }
