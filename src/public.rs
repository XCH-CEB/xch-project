// Copyright 2017-2018 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::str::FromStr;
use std::marker::Copy;
use std::fmt::Debug;
use num_traits::Num;
// inside uses
use handler::ErrorCases::{AbsError, NegError, Overflow};
use handler::ErrorCases;

// Operator
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Abs,
    Neg,
}

/// This is the trait for `safe_calc` and whole lib's 'meta-calc-part'.
pub trait CheckedType
    : Num
    + Copy
    + Debug
    + Ord
    + FromStr
    + ToString
    + Clone
    + CheckedAdd
    + CheckedSub
    + CheckedMul
    + CheckedDiv
    + CheckedRem
    + CheckedAbs
    + CheckedNeg {
    // Empty
}

pub trait CheckedAdd: Sized {
    fn checked_add(&self, rhs: &Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(&self, rhs: &Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(&self, rhs: &Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedDiv: Sized {
    fn checked_div(&self, rhs: &Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedRem: Sized {
    fn checked_rem(&self, rhs: &Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedAbs: Sized {
    fn checked_abs(&self) -> Result<Self, ErrorCases>;
}

pub trait CheckedNeg: Sized {
    fn checked_neg(&self) -> Result<Self, ErrorCases>;
}

// marcos for auto-creating implementations
macro_rules! checked_impl_double {
    ($trait_name:ident, $method:ident, $t:ty, $error_type: ident) => {
        impl $trait_name for $t {
            fn $method(&self, rhs: &$t) -> Result<Self, ErrorCases> {
                <$t>::$method(*self, *rhs).ok_or($error_type)
            }
        }
    }
}

macro_rules! checked_impl_single {
    ($trait_name:ident, $method:ident, $t:ty, $error_type: ident) => {
        impl $trait_name for $t {
            fn $method(&self) -> Result<Self, ErrorCases> {
                <$t>::$method(*self).ok_or($error_type)
            }
        }
    }
}

pub fn safe_calc<T: CheckedType>(a: &T, b: &T, op: &Operator) -> Result<T, ErrorCases> {
    match *op {
        Operator::Add => CheckedAdd::checked_add(a, b),
        Operator::Sub => CheckedSub::checked_sub(a, b),
        Operator::Mul => CheckedMul::checked_mul(a, b),
        Operator::Div => CheckedDiv::checked_div(a, b),
        Operator::Rem => CheckedRem::checked_rem(a, b),
        Operator::Abs => CheckedAbs::checked_abs(a),
        Operator::Neg => CheckedNeg::checked_neg(a),
    }
}

// Implementations on usize
impl CheckedType for usize {}
checked_impl_double!(CheckedAdd, checked_add, usize, Overflow);
checked_impl_double!(CheckedSub, checked_sub, usize, Overflow);
checked_impl_double!(CheckedMul, checked_mul, usize, Overflow);
checked_impl_double!(CheckedDiv, checked_div, usize, Overflow);
checked_impl_double!(CheckedRem, checked_rem, usize, Overflow);
checked_impl_single!(CheckedNeg, checked_neg, usize, NegError);
impl CheckedAbs for usize {
    fn checked_abs(&self) -> Result<Self, ErrorCases> {
        Ok(*self)
    }
}

// Implementations on i32
impl CheckedType for i32 {}
checked_impl_double!(CheckedAdd, checked_add, i32, Overflow);
checked_impl_double!(CheckedSub, checked_sub, i32, Overflow);
checked_impl_double!(CheckedMul, checked_mul, i32, Overflow);
checked_impl_double!(CheckedDiv, checked_div, i32, Overflow);
checked_impl_double!(CheckedRem, checked_rem, i32, Overflow);
checked_impl_single!(CheckedAbs, checked_abs, i32, AbsError);
checked_impl_single!(CheckedNeg, checked_neg, i32, NegError);
