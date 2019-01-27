// Copyright 2017-2019 LEXUGE
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

//! The traits which may be useful.

use nalgebra::base::Scalar;
use num::{
    traits::{
        ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub},
        Num, NumAssign,
    },
    Integer, Signed,
};
// inside use(s)
use std::{fmt::Display, marker::Copy, str::FromStr};

// marcos for auto-creating implementations
macro_rules! checked_calc_impl {
    ($t:ty) => {
        impl CheckedCalc for $t {}
    };
}

macro_rules! checked_type_impl {
    ($t:ty) => {
        impl CheckedType for $t {}
    };
}

/// The trait which must be implemented when using `safe_calc()`
pub trait CheckedCalc:
    CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + CheckedNeg
{
    // Empty
}

/// The trait which must be implemented.
pub trait CheckedType:
    Num + Copy + FromStr + Integer + Scalar + Signed + NumAssign + Display + Clone
{
    // Empty
}

// Implementations on Primitive types
checked_calc_impl!(u8);
checked_calc_impl!(u16);
checked_calc_impl!(u32);
checked_calc_impl!(u64);
checked_calc_impl!(u128);
checked_calc_impl!(usize);

checked_calc_impl!(i8);
checked_calc_impl!(i16);
checked_calc_impl!(i32);
checked_calc_impl!(i64);
checked_calc_impl!(i128);
checked_calc_impl!(isize);

checked_type_impl!(i8);
checked_type_impl!(i16);
checked_type_impl!(i32);
checked_type_impl!(i64);
checked_type_impl!(i128);
checked_type_impl!(isize);
