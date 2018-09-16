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

use num::traits::ops::checked::{CheckedAdd, CheckedNeg};
// inside uses
use api::{
    handler::{
        ErrorCases,
        ErrorCases::{NegError, Overflow},
    },
    traits::CheckedCalc,
};

// Operator
pub enum Operator {
    Add,
    Neg,
}

pub fn safe_calc<T: CheckedCalc>(a: &T, b: &T, op: &Operator) -> Result<T, ErrorCases> {
    match *op {
        Operator::Add => CheckedAdd::checked_add(a, b).ok_or(Overflow),
        Operator::Neg => CheckedNeg::checked_neg(a).ok_or(NegError),
    }
}
