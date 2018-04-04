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

use handler::ErrorCases::{AbsError, Overflow};
use handler::ErrorCases;

// Operator
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Abs,
}

// 'checked' traits
pub trait CheckedAdd: Sized {
    fn checked_add(self, rhs: Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(self, rhs: Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(self, rhs: Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedDiv: Sized {
    fn checked_div(self, rhs: Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedRem: Sized {
    fn checked_rem(self, rhs: Self) -> Result<Self, ErrorCases>;
}

pub trait CheckedAbs: Sized {
    fn checked_abs(self) -> Result<Self, ErrorCases>;
}

macro_rules! impl_checked_traits {
    ($($ty:ident),*) => {
        $(impl CheckedAdd for $ty {
            fn checked_add(self, rhs: $ty) -> Result<Self, ErrorCases> {
                $ty::checked_add(self, rhs).ok_or(Overflow)
            }
        }
        impl CheckedSub for $ty {
            fn checked_sub(self, rhs: $ty) -> Result<Self, ErrorCases> {
                $ty::checked_sub(self, rhs).ok_or(Overflow)
            }
        }
        impl CheckedMul for $ty {
            fn checked_mul(self, rhs: $ty) -> Result<Self, ErrorCases> {
                $ty::checked_mul(self, rhs).ok_or(Overflow)
            }
        }
        impl CheckedDiv for $ty {
            fn checked_div(self, rhs: $ty) -> Result<Self, ErrorCases> {
                $ty::checked_div(self, rhs).ok_or(Overflow)
            }
        }
        impl CheckedRem for $ty {
            fn checked_rem(self, rhs: $ty) -> Result<Self, ErrorCases> {
                $ty::checked_rem(self, rhs).ok_or(Overflow)
            }
        }
        impl CheckedAbs for $ty {
            fn checked_abs(self) -> Result<Self, ErrorCases> {
                $ty::checked_abs(self).ok_or(AbsError)
            }
        })*
    };
}

pub fn safe_calc<T>(a: T, b: T, op: &Operator) -> Result<T, ErrorCases>
where
    T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + CheckedAbs,
{
    match *op {
        Operator::Add => CheckedAdd::checked_add(a, b),
        Operator::Sub => CheckedSub::checked_sub(a, b),
        Operator::Mul => CheckedMul::checked_mul(a, b),
        Operator::Div => CheckedDiv::checked_div(a, b),
        Operator::Rem => CheckedRem::checked_rem(a, b),
        Operator::Abs => CheckedAbs::checked_abs(a),
    }
}

impl_checked_traits!(i32);
