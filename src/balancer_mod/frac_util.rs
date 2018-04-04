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

// Overall: This is the source code of the AlphaForce Balancer.

use std::cmp::Ordering;
use std::ops;
// inside uses
use super::math_methods::{gcd, lcm};
use handler::ErrorCases::UndefinedFrac;
use handler::ErrorCases;
use public::Operator::{Abs, Add, Div, Mul, Sub};
use public::safe_calc;

fn rfcd(a: &Frac, b: &Frac) -> Result<(i32, i32, i32), ErrorCases> {
    // reduction of fractions to a common denominator
    let d = lcm(a.denominator, b.denominator)?;
    let mut a_n = safe_calc(d, a.denominator, &Div)?;
    let mut b_n = safe_calc(d, b.denominator, &Div)?;
    a_n = safe_calc(a_n, a.numerator, &Mul)?;
    b_n = safe_calc(b_n, b.numerator, &Mul)?;
    Ok((a_n, b_n, d))
}

#[derive(Clone, Copy)]
pub struct Frac {
    pub numerator: i32,
    pub denominator: i32,
}

impl Frac {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn check(&self) -> Result<bool, ErrorCases> {
        if self.denominator == 0 {
            Err(UndefinedFrac)
        } else {
            Ok(true)
        }
    }

    pub fn simple(&self) -> Result<Self, ErrorCases> {
        self.check()?;
        let gcd = gcd(self.numerator, self.denominator)?;
        Ok(Self {
            numerator: safe_calc(self.numerator, gcd, &Div)?,
            denominator: safe_calc(self.denominator, gcd, &Div)?,
        })
    }

    pub fn abs(&self) -> Result<Self, ErrorCases> {
        self.check()?;
        let mut tmp = Self {
            numerator: safe_calc(self.numerator, 0, &Abs)?,
            denominator: safe_calc(self.denominator, 0, &Abs)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl ops::Add for Frac {
    type Output = Result<Self, ErrorCases>;
    fn add(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let (a_n, b_n, d) = rfcd(&self, &b)?;
        let mut tmp = Self {
            numerator: safe_calc(a_n, b_n, &Add)?,
            denominator: d,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl ops::Sub for Frac {
    type Output = Result<Self, ErrorCases>;
    fn sub(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let (a_n, b_n, d) = rfcd(&self, &b)?;
        let mut tmp = Self {
            numerator: safe_calc(a_n, b_n, &Sub)?,
            denominator: d,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl ops::Mul for Frac {
    type Output = Result<Self, ErrorCases>;
    fn mul(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let mut tmp = Self {
            numerator: safe_calc(self.numerator, b.numerator, &Mul)?,
            denominator: safe_calc(self.denominator, b.denominator, &Mul)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl ops::Div for Frac {
    type Output = Result<Self, ErrorCases>;
    fn div(self, b: Self) -> Result<Self, ErrorCases> {
        self.check()?;
        b.check()?;
        let mut tmp = Self {
            numerator: safe_calc(self.numerator, b.denominator, &Mul)?,
            denominator: safe_calc(self.denominator, b.numerator, &Mul)?,
        };
        tmp = tmp.simple()?;
        tmp.check()?;
        Ok(tmp)
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        if self.check().is_err() || b.check().is_err() {
            None
        } else {
            let d = match lcm(self.denominator, b.denominator) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] `lcm` Err: {:?}", e),
            };
            let mut a_n = match safe_calc(d, self.denominator, &Div) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            let mut b_n = match safe_calc(d, b.denominator, &Div) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            a_n = match safe_calc(a_n, self.numerator, &Mul) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            b_n = match safe_calc(b_n, b.numerator, &Mul) {
                Ok(s) => s,
                Err(e) => panic!("[Ord] Err: {:?}", e),
            };
            Some(a_n.cmp(&b_n))
        }
    }
}

impl PartialEq for Frac {
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
