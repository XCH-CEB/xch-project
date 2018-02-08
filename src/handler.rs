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

// inside uses
use parser_mod::xch_parser;
use balancer_mod::xch_try;

/// the API balances the Chemical Equation by equation and searching range.
/// If the equation can balance, function would return a `i32` vector which contains the answer.
/// If not, it would return a `Vec<Vec<i32>>` which contains Delta-3 the parser's result.
///
/// # Panics
///
/// The equation you provided should be a common unbalanced chemical equation which only contains **one** `+`.
/// In following cases, API will **panic**:
/// 1.Contians illegal chars. Equation should only contain `+`, `=`, letters, numbers.
/// 2.Contians **more than one** `=`.
/// 3.Unmatched `(` and `)`.
/// 4.Several kinds of `i32 overflow`.
/// 5.Stack Overflow may cause panic too. Because we are using recusive balancer and regex-based parser.
pub fn handler_api(equation: String, searching_range: i32) -> Result<Vec<i32>, Vec<Vec<i32>>> {
    // T is successful traversal vector, E is list vector which parser returned.
    let mut traversal: Vec<i32> = Vec::new();
    let (chemical_equation_struct, elements_table, list) = xch_parser(equation);
    match xch_try(
        1,
        searching_range,
        &mut traversal,
        &list,
        &chemical_equation_struct,
        elements_table.len(),
    ) {
        true => Ok(traversal),
        false => Err(list),
    }
}
