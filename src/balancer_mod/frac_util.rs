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

use super::public_methods::{gcd, lcm};
use std::cmp::Ordering;
use handler::ErrorCases::{I32AbsError, I32Overflow};
use handler::ErrorCases;

#[derive(PartialEq, Clone, Copy, Eq)]
pub struct Frac {
    pub numerator: i32,
    pub denominator: i32,
}

impl Frac {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator: numerator,
            denominator: denominator,
        }
    }

    pub fn simple(&mut self) -> Result<bool, ErrorCases> {
        let gcd = gcd(self.numerator, self.denominator)?;
        self.numerator = match self.numerator.checked_div(gcd) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        self.denominator = match self.denominator.checked_div(gcd) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        Ok(true)
    }

    pub fn abs(&self) -> Result<Frac, ErrorCases> {
        let numerator = match self.numerator.checked_abs() {
            Some(s) => s,
            None => return Err(I32AbsError),
        };
        let denominator = match self.denominator.checked_abs() {
            Some(s) => s,
            None => return Err(I32AbsError),
        };
        Ok(Frac {
            numerator: numerator,
            denominator: denominator,
        })
    }

    pub fn add(self, other: Frac) -> Result<Frac, ErrorCases> {
        let denominator = lcm(self.denominator, other.denominator)?;
        let this_numerator = match denominator.checked_div(self.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let other_numerator = match denominator.checked_div(other.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let this_numerator = match this_numerator.checked_mul(self.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let other_numerator = match other_numerator.checked_mul(other.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let this_numerator = match this_numerator.checked_add(other_numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let mut tmp = Frac {
            numerator: this_numerator,
            denominator: denominator,
        };
        tmp.simple()?;
        Ok(tmp)
    }

    pub fn sub(self, other: Frac) -> Result<Frac, ErrorCases> {
        let denominator = lcm(self.denominator, other.denominator)?;
        let this_numerator = match denominator.checked_div(self.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let other_numerator = match denominator.checked_div(other.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let this_numerator = match this_numerator.checked_mul(self.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let other_numerator = match other_numerator.checked_mul(other.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let this_numerator = match this_numerator.checked_sub(other_numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let mut tmp = Frac {
            numerator: this_numerator,
            denominator: denominator,
        };
        tmp.simple()?;
        Ok(tmp)
    }

    pub fn mul(self, other: Frac) -> Result<Frac, ErrorCases> {
        let numerator = match self.numerator.checked_mul(other.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let denominator = match self.denominator.checked_mul(other.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let mut tmp = Frac {
            numerator: numerator,
            denominator: denominator,
        };
        tmp.simple()?;
        Ok(tmp)
    }

    pub fn div(&mut self, other: Frac) -> Result<Frac, ErrorCases> {
        let numerator = match self.numerator.checked_mul(other.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let denominator = match self.denominator.checked_mul(other.numerator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        let mut tmp = Frac {
            numerator: numerator,
            denominator: denominator,
        };
        tmp.simple()?;
        Ok(tmp)
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Frac) -> Ordering {
        let denominator = lcm(self.denominator, other.denominator).expect("[Ord] LCM Error");
        let this_numerator = match denominator.checked_div(self.denominator) {
            Some(s) => s,
            None => panic!("[Ord] i32 Overflow"),
        };
        let other_numerator = match denominator.checked_div(other.denominator) {
            Some(s) => s,
            None => panic!("[Ord] i32 Overflow"),
        };
        let this_numerator = match this_numerator.checked_mul(self.numerator) {
            Some(s) => s,
            None => panic!("[Ord] i32 Overflow"),
        };
        let other_numerator = match other_numerator.checked_mul(other.numerator) {
            Some(s) => s,
            None => panic!("[Ord] i32 Overflow"),
        };
        this_numerator.cmp(&other_numerator)
    }
}
