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
/// If not, it would return `handler::ErrorHandler` which contains Delta-3 the parser's result and error message.
///
/// # Panics
///
/// The equation you provided should be a common unbalanced chemical equation which only contains **one** `+`.
/// In following cases, API will **panic**:
/// 1.Stack Overflow may cause panic too. Because it is using recusive balancer and regex-based parser.
/// And in the other failed situation, it'll return a `error_message` and contain `parser_result`(maybe it is empty).
pub fn handler_api(equation: String, searching_range: i32) -> Result<Vec<i32>, ErrorHandler> {
    // T is successful traversal vector, E is list vector which parser returned.
    let mut traversal: Vec<i32> = Vec::new();
    let (chemical_equation_struct, elements_table, list) = match xch_parser(equation) {
        Ok(some) => some,
        Err(e) => {
            return Err(ErrorHandler {
                error_message: e,
                parser_result: {
                    let list: Vec<Vec<i32>> = Vec::new();
                    list
                },
            })
        }
    };
    match xch_try(
        1,
        searching_range,
        &mut traversal,
        &list,
        &chemical_equation_struct,
        elements_table.len(),
    ) {
        Ok(true) => Ok(traversal),
        Ok(false) => Err(ErrorHandler {
            error_message: "No answer".to_string(),
            parser_result: list,
        }),
        Err(s) => Err(ErrorHandler {
            error_message: s,
            parser_result: list,
        }),
    }
}

/// ErrorHandler returns when `handler::handler_api` failed somehow.
/// **CAUTION: parser_result might empty if parser is failed.**
pub struct ErrorHandler {
    pub error_message: String,
    pub parser_result: Vec<Vec<i32>>,
}
