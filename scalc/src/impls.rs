// Copyright 2019 LEXUGE
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

// use(s)
use {
    crate::SCell,
    num_traits::{
        CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, Num, One, Signed,
        Zero,
    },
    std::{
        cmp::Ordering,
        ops::{Add, Div, Mul, Rem, Sub},
    },
};

// Basic implementations
impl<T> SCell<T> {
    /// Create a new `SCell`
    pub fn new(data: T) -> Self {
        SCell {
            error_tag: false,
            data,
        }
    }
    /// Get the status which denotes that whether it's overflowed or not  
    /// `true` for overflowed and `false` for normal status
    pub fn is_overflowed(&self) -> bool {
        self.error_tag
    }
    /// Get the data of underlying type `T`
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

// Implementations of `Eq`, `PartialEq`, `Ord`, `PartialOrd`
impl<T: PartialEq> PartialEq for SCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: PartialEq> Eq for SCell<T> {}

impl<T: PartialOrd + Ord> Ord for SCell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<T: PartialEq + Ord> PartialOrd for SCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

// Implementations of `One` and `Zero`
impl<T: Zero + One + CheckedAdd> Zero for SCell<T> {
    fn zero() -> Self {
        SCell {
            error_tag: false,
            data: T::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.data.is_zero()
    }
}
impl<T: One + CheckedMul + PartialEq> One for SCell<T> {
    fn one() -> Self {
        SCell {
            error_tag: false,
            data: T::one(),
        }
    }
    fn is_one(&self) -> bool {
        self.data.is_one()
    }
}

// Implementations of `Ops`
ops!(+=, add, Add);
ops!(-=, sub, Sub);
ops!(*=, mul, Mul);
ops!(/=, div, Div);
ops!(%=, rem, Rem);
ops!(neg);

// Implementation of `Num`
impl<T: Num + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem> Num for SCell<T> {
    type FromStrRadixErr = T::FromStrRadixErr;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let data = T::from_str_radix(str, radix)?;
        Ok(SCell {
            error_tag: false,
            data,
        })
    }
}

// Implementation of `Signed`
impl<
        T: Signed
            + CheckedAdd
            + CheckedSub
            + CheckedMul
            + CheckedDiv
            + CheckedRem
            + CheckedNeg
            + Ord
            + Copy,
    > Signed for SCell<T>
{
    fn abs(&self) -> Self {
        if *self < Self::zero() {
            -*self
        } else {
            *self
        }
    }
    fn abs_sub(&self, other: &Self) -> Self {
        if *self <= *other {
            Self::zero()
        } else {
            *self - *other
        }
    }
    fn signum(&self) -> Self {
        SCell {
            error_tag: self.error_tag,
            data: self.data.signum(),
        }
    }
    fn is_positive(&self) -> bool {
        self.data.is_positive()
    }
    fn is_negative(&self) -> bool {
        self.data.is_negative()
    }
}
