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
    std::ops::{Add, Div, Mul, Rem, Sub},
};

// Basic implementations
impl<T> SCell<T> {
    /// Create a new `SCell`
    pub fn new(data: T) -> Self {
        SCell { data }
    }
    /// Get the data of underlying type `T`
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

// Implementations of methods which are given in `One` and `Zero`
impl<T: Zero> SCell<T> {
    /// See `num` crate for more information
    pub fn zero() -> Self {
        SCell { data: T::zero() }
    }
    /// See `num` crate for more information
    pub fn is_zero(&self) -> bool {
        self.data.is_zero()
    }
}
impl<T: One + PartialEq> SCell<T> {
    /// See `num` crate for more information
    pub fn one() -> Self {
        SCell { data: T::one() }
    }
    /// See `num` crate for more information
    pub fn is_one(&self) -> bool {
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

// Implementation of method which is given in `Num`
impl<T: Num + CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem> SCell<T> {
    /// See `num` crate for more information
    pub fn from_str_radix(str: &str, radix: u32) -> Result<Self, T::FromStrRadixErr> {
        let data = T::from_str_radix(str, radix)?;
        Ok(SCell { data })
    }
}

// Implementation of methods which are given in `Signed`
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
    > SCell<T>
{
    /// See `num` crate for more information
    pub fn abs(&self) -> Option<Self> {
        if *self < Self::zero() {
            -*self
        } else {
            Some(*self)
        }
    }
    /// See `num` crate for more information
    pub fn abs_sub(&self, other: &Self) -> Option<Self> {
        if *self <= *other {
            Some(Self::zero())
        } else {
            *self - *other
        }
    }
    /// See `num` crate for more information
    pub fn signum(&self) -> Self {
        SCell {
            data: self.data.signum(),
        }
    }
    /// See `num` crate for more information
    pub fn is_positive(&self) -> bool {
        self.data.is_positive()
    }
    /// See `num` crate for more information
    pub fn is_negative(&self) -> bool {
        self.data.is_negative()
    }
}
