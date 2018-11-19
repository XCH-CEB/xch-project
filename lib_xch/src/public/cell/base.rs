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

use num::{One, Zero};
use std::{
    fmt::{Display, Error, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
// inside use(s)
use super::Cell;
use api::traits::CheckedCalc;
use public::calc::{safe_calc, Operator};

// `Zero` and `One` impls
impl<U: One + Zero + CheckedCalc> Zero for Cell<U> {
    fn zero() -> Self {
        Cell {
            error_tag: false,
            data: U::zero(),
        }
    }
    fn is_zero(&self) -> bool {
        self.data.is_zero()
    }
}
impl<U: One + Zero + CheckedCalc + PartialEq> One for Cell<U> {
    fn one() -> Self {
        Cell {
            error_tag: false,
            data: U::one(),
        }
    }
    fn is_one(&self) -> bool {
        self.data.is_one()
    }
}

// NumAssignOps
// Using `One` instead of `Zero` in case of dividing zero
impl<U: One + CheckedCalc> AddAssign for Cell<U> {
    fn add_assign(&mut self, rhs: Self) {
        self.data = match safe_calc(&self.data, &rhs.data, &Operator::Add) {
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        self.error_tag |= rhs.error_tag;
    }
}
impl<U: One + CheckedCalc> SubAssign for Cell<U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.data = match safe_calc(&self.data, &rhs.data, &Operator::Sub) {
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        self.error_tag |= rhs.error_tag;
    }
}
impl<U: One + CheckedCalc> MulAssign for Cell<U> {
    fn mul_assign(&mut self, rhs: Self) {
        self.data = match safe_calc(&self.data, &rhs.data, &Operator::Mul) {
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        self.error_tag |= rhs.error_tag;
    }
}
impl<U: One + CheckedCalc> DivAssign for Cell<U> {
    fn div_assign(&mut self, rhs: Self) {
        self.data = match safe_calc(&self.data, &rhs.data, &Operator::Div) {
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        self.error_tag |= rhs.error_tag;
    }
}
impl<U: One + CheckedCalc> RemAssign for Cell<U> {
    fn rem_assign(&mut self, rhs: Self) {
        self.data = match safe_calc(&self.data, &rhs.data, &Operator::Rem) {
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        self.error_tag |= rhs.error_tag;
    }
}

// NumOps
impl<U: One + CheckedCalc> Add for Cell<U> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl<U: One + CheckedCalc> Sub for Cell<U> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<U: One + CheckedCalc> Mul for Cell<U> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<U: One + CheckedCalc> Div for Cell<U> {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}
impl<U: One + CheckedCalc> Rem for Cell<U> {
    type Output = Self;
    fn rem(mut self, rhs: Self) -> Self::Output {
        self %= rhs;
        self
    }
}
impl<U: One + CheckedCalc> Neg for Cell<U> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.data = match safe_calc(&self.data, &U::one(), &Operator::Neg) {
            // The value of the 'b' position is useless
            Ok(s) => s,
            Err(_) => {
                self.error_tag = true;
                U::one()
            }
        };
        Cell {
            error_tag: self.error_tag,
            data: self.data,
        }
    }
}

// Display
impl<U: Display> Display for Cell<U> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        std::fmt::Display::fmt(&self.data, f)?;
        Ok(())
    }
}
