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

use std::ops::Rem;
use std::str::FromStr;
use std::marker::Copy;
use std::fmt::Debug;
use num_traits::Num;
use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

// marcos for auto-creating implementations
macro_rules! checked_impl_double {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            fn $method(&self, v: &$t) -> Option<$t> {
                <$t>::$method(*self, *v)
            }
        }
    }
}

macro_rules! checked_impl_single {
    ($trait_name:ident, $method:ident, $t:ty) => {
        impl $trait_name for $t {
            fn $method(&self) -> Option<$t> {
                <$t>::$method(*self)
            }
        }
    }
}

macro_rules! checked_impl {
    (int,$t:ty) => {
        impl CheckedType for $t {}
        checked_impl_double!(CheckedRem, checked_rem, $t);
        checked_impl_single!(CheckedAbs, checked_abs, $t);
        checked_impl_single!(CheckedNeg, checked_neg, $t);
    };
    (unsigned,$t:ty) => {
        impl CheckedType for $t {}
        checked_impl_double!(CheckedRem, checked_rem, $t);
        checked_impl_single!(CheckedNeg, checked_neg, $t);
        impl CheckedAbs for $t {
            fn checked_abs(&self) -> Option<Self> {
                Some(*self)
            }
        }
    };
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

pub trait CheckedRem: Sized + Rem<Self, Output = Self> {
    fn checked_rem(&self, v: &Self) -> Option<Self>;
}

pub trait CheckedAbs: Sized {
    fn checked_abs(&self) -> Option<Self>;
}

pub trait CheckedNeg: Sized {
    fn checked_neg(&self) -> Option<Self>;
}

// Implementations on Primitive types
checked_impl!(unsigned, u8);
checked_impl!(unsigned, u16);
checked_impl!(unsigned, u32);
checked_impl!(unsigned, u64);
checked_impl!(unsigned, usize);

checked_impl!(int, i8);
checked_impl!(int, i16);
checked_impl!(int, i32);
checked_impl!(int, i64);
checked_impl!(int, isize);
