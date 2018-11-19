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

//! The major part of APIs.

// inside uses
use super::{
    structs::ChemicalEquation,
    traits::{CheckedCalc, CheckedType},
};
use balancer::handler::xch_balancer;
use parser::handler::parser;
use public::cell::Cell;

// type aliases
type Error<T> = (ErrorCases, Vec<Vec<T>>);
type Normal<T> = (ChemicalEquation, Vec<Vec<T>>);

/// The API which balances the Chemical Equation by equation.
///
/// It provides a set of Basic Solutions. You can get infinite number of solutions by doing linear combination on the Basic Solution Set.
///
/// You can use any type which implemented the trait `api::traits::CheckedType`
///
/// If the equation can balance, function would return `Ok((ChemicalEquation, Vec<Vec<T>>))` which contains the answer.
///
/// If not, it would return `Err((ErrorCases, Vec<Vec<T>>))` which contains the error message and Delta-3 the parser's result.
///
/// # Panics
///
/// -  A large number (bigger than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/usize/constant.MAX.html)) of formula may cause **panic**. Because it is using `Vec`.
///
/// And in the other failed situation, it'll return  `ErrorCases` and  parser's result (maybe it is empty).
pub fn handler_api<T: CheckedType + CheckedCalc>(equation: &str) -> Result<Normal<T>, Error<T>>
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    let (ce_desc, list) = match parser::<Cell<T>>(equation) {
        Ok(s) => s,
        Err(e) => return Err((e, Vec::new())),
    };
    match xch_balancer(&list, &ce_desc) {
        Ok(s) => if check_tag(&s) {
            Ok((ce_desc, fromcell(&s)))
        } else {
            Err((ErrorCases::Overflow, fromcell(&list)))
        },
        Err(e) => if check_tag(&list) {
            Err((e, fromcell(&list)))
        } else {
            Err((ErrorCases::Overflow, fromcell(&list)))
        },
    }
}

// All `false` => `true`
fn check_tag<T>(s: &[Vec<Cell<T>>]) -> bool {
    s.iter().all(|x| x.iter().all(|x| !x.get_tag()))
}

fn fromcell<T: Clone>(s: &[Vec<Cell<T>>]) -> Vec<Vec<T>> {
    s.into_iter()
        .map(|x| x.into_iter().map(|s| s.get_data()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

/// All the Error Types.
#[derive(PartialEq, Debug)]
pub enum ErrorCases {
    /// Overflow.
    Overflow,
    /// Parser's error with a message.
    ParserError(String),
    /// `checked_neg()` error.
    NegError,
    /// Only the [zero solution](http://www.mathwords.com/t/trivial.htm) can be found.
    ZeroSolution,
}
