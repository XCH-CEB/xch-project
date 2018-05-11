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

// Overall: This is the source code of the Hyper Mathlib.

use std::cmp::Ordering;
use std::ops;
// inside uses
use super::basic_fns::{gcd, lcm};
use api::handler::ErrorCases;
use api::handler::ErrorCases::UndefinedFrac;
use api::traits::CheckedType;
use public::Operator::{Abs, Add, Div, Mul, Sub};
use public::safe_calc;

fn rfcd<T: CheckedType>(a: &Frac<T>, b: &Frac<T>) -> Result<(T, T, T), ErrorCases> {
    // reduction of fractions to a common denominator
    let d = lcm(a.denominator, b.denominator)?;
    let mut a_n = safe_calc(&d, &a.denominator, &Div)?;
    let mut b_n = safe_calc(&d, &b.denominator, &Div)?;
    a_n = safe_calc(&a_n, &a.numerator, &Mul)?;
    b_n = safe_calc(&b_n, &b.numerator, &Mul)?;
    Ok((a_n, b_n, d))
}

#[derive(Clone, Copy)]
pub struct Frac<T: CheckedType> {
    pub numerator: T,
    pub denominator: T,
}

impl<T: CheckedType> Frac<T> {
    pub fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn check(&self) -> Result<bool, ErrorCases> {
        if self.denominator == T::zero() {
            Err(UndefinedFrac)
        } else {
            Ok(true)
        }
    }

    pub fn simple(&self) -> Result<Self, ErrorCases> {
        self.check()?;
        let gcd = gcd(self.numerator, self.denominator)?;
        Ok(Self {
            numerator: safe_calc(&self.numerator, &gcd, &Div)?,
            denominator: safe_calc(&self.denominator, &gcd, &Div)?,
        })
    }

    pub fn abs(&self) -> Result<Self, ErrorCases> {
        self.check()?;
        let mut tmp = Self {
            numerator: safe_calc(&self.numerator, &T::zero(), &Abs)?,
            denominator: safe_calc(&self.denominator, &T::zero(), &Abs)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl<T: CheckedType> ops::Add for Frac<T> {
    type Output = Result<Self, ErrorCases>;
    fn add(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let (a_n, b_n, d) = rfcd(&self, &b)?;
        let mut tmp = Self {
            numerator: safe_calc(&a_n, &b_n, &Add)?,
            denominator: d,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl<T: CheckedType> ops::Sub for Frac<T> {
    type Output = Result<Self, ErrorCases>;
    fn sub(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let (a_n, b_n, d) = rfcd(&self, &b)?;
        let mut tmp = Self {
            numerator: safe_calc(&a_n, &b_n, &Sub)?,
            denominator: d,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl<T: CheckedType> ops::Mul for Frac<T> {
    type Output = Result<Self, ErrorCases>;
    fn mul(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let mut tmp = Self {
            numerator: safe_calc(&self.numerator, &b.numerator, &Mul)?,
            denominator: safe_calc(&self.denominator, &b.denominator, &Mul)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl<T: CheckedType> ops::Div for Frac<T> {
    type Output = Result<Self, ErrorCases>;
    fn div(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let mut tmp = Self {
            numerator: safe_calc(&self.numerator, &b.denominator, &Mul)?,
            denominator: safe_calc(&self.denominator, &b.numerator, &Mul)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl<T: CheckedType> PartialOrd for Frac<T> {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        if self.check().is_err() || b.check().is_err() {
            None
        } else {
            let d = match lcm(self.denominator, b.denominator) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] `lcm` Err: {:?}", e),
            };
            let mut a_n = match safe_calc(&d, &self.denominator, &Div) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            let mut b_n = match safe_calc(&d, &b.denominator, &Div) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            a_n = match safe_calc(&a_n, &self.numerator, &Mul) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            b_n = match safe_calc(&b_n, &b.numerator, &Mul) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            Some(a_n.cmp(&b_n))
        }
    }
}

impl<T: CheckedType> PartialEq for Frac<T> {
    fn eq(&self, b: &Self) -> bool {
        if b.check().is_err() || self.check().is_err() {
            panic!("[Eq] UndefinedFrac")
        }
        let a = match self.simple() {
            Ok(s) => s,
            Err(e) => panic!("[Eq] Err: {:?}", e),
        };
        // shadow `b`
        let b = match b.simple() {
            Ok(s) => s,
            Err(e) => panic!("[Eq] Err: {:?}", e),
        };
        (a.numerator == b.numerator) && (a.denominator == b.denominator)
    }
}
