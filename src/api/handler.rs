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
use api::traits::CheckedType;
use balancer_mod::xch_balancer;
use parser_mod::xch_parser;

// type aliases
type Error<T> = (ErrorCases, Vec<Vec<T>>);

/// The API which balances the Chemical Equation by equation.
///
/// It provides a set of Basic Solutions. You can get infinite number of solutions by doing linear combination on the Basic Solution Set.
///
/// You can use any type which implemented the trait `api::traits::CheckedType`
///
/// If the equation can balance, function would return a `Vec<Vec<T>>` which contains the answer.
///
/// If not, it would return `Err((ErrorCases, Vec<Vec<T>>))` which contains the error message and Delta-3 the parser's result.
///
/// # Panics
///
/// The equation you provided should be a common unbalanced chemical equation which only contains **one** `=`.
///
/// -  Stack Overflow may cause **panic**. Because it is using regex-based parser.
/// -  A large number (bigger than [`usize::MAX`](https://doc.rust-lang.org/nightly/std/usize/constant.MAX.html)) of formula may cause **panic**. Because it is using `Vec`.
///
/// And in the other failed situation, it'll return  `ErrorCases` and  parser's result (maybe it is empty).
pub fn handler_api<T: CheckedType>(equation: &str) -> Result<Vec<Vec<T>>, Error<T>> {
    let (ce_desc, list) = match xch_parser(equation) {
        Ok(s) => s,
        Err(e) => return Err((e, Vec::new())),
    };
    match xch_balancer(&list, &ce_desc) {
        Ok(s) => Ok(s),
        Err(e) => Err((e, list)),
    }
}

/// All the Error Types.
#[derive(PartialEq, Debug)]
pub enum ErrorCases {
    /// More or less than 1 `=` or not allowed chars.
    IllegalEquation,
    /// Overflow.
    Overflow,
    /// Brackets are not matched.
    MatchError,
    /// No formulas to split.
    SplitError,
    /// No tokens to get.
    NoTokens,
    /// Not found in `elements_table`.
    NotFound,
    /// Can't parse into `T`.
    ParseError,
    /// `checked_neg()` error.
    NegError,
    /// Internal Error - Illegal Usage was happened.
    IllegalUsage,
    /// Only the [zero solution](http://www.mathwords.com/t/trivial.htm) can be found.
    ZeroSolution,
}
