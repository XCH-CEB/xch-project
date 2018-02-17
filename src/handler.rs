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
use balancer_mod::xch_balancer;

/// the API balances the Chemical Equation by equation.
/// It provides one balanced solution, but it may isn't the *most* correct solution.
/// If the equation can balance, function would return a `i32` vector which contains the answer.
/// If not, it would return `handler::ErrorHandler` which contains Delta-3 the parser's result and error message.
///
/// # Panics
///
/// The equation you provided should be a common unbalanced chemical equation which only contains **one** `=`.
///
/// -  Stack Overflow may cause **panic**. Because it is using regex-based parser.
/// -  The implement for Ord trait may cause **panic**. Because it should return `Ordering`.
///
/// And in the other failed situation, it'll return a `error_message` and contain `parser_result`(maybe it is empty).
pub fn handler_api(equation: &str) -> Result<ResultHandler, ErrorHandler> {
    // T is successful traversal vector, E is list vector which parser returned.
    let (chemical_equation_struct, list) = match xch_parser(equation) {
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
    match xch_balancer(&list, &chemical_equation_struct) {
        Ok(s) => Ok(s),
        Err(e) => Err(ErrorHandler {
            error_message: e,
            parser_result: list,
        }),
    }
}

/// `ErrorHandler` returns when `handler::handler_api` failed somehow.
/// **CAUTION: `parser_result` might empty if parser is failed.**
pub struct ErrorHandler {
    pub error_message: ErrorCases,
    pub parser_result: Vec<Vec<i32>>,
}

/// `ResultHandler` returns the balancer's result.
/// And it may contain warning message.
pub struct ResultHandler {
    pub warn_message: WarnCases,
    pub result: Vec<i32>,
}

/// All the Error Types.
///
/// -  more or less than 1 `=`; not allowed chars; too many formulas.
/// -  i32 overflow.
/// -  brackets are not matched.
/// -  no formulas to split.
/// -  no tokens to get.
/// -  not found in `elements_table`.
/// -  no answer.
/// -  Can't parse into i32.
/// -  Contians Free Variables.
/// -  Equation set unsolvable.
/// -  i32 `checked_abs()` error.
#[derive(PartialEq, Debug)]
pub enum ErrorCases {
    IllegalEquation,
    I32Overflow,
    MatchError,
    SplitError,
    NoTokens,
    NotFound,
    NoAnswer,
    I32ParseError,
    FreeVariables,
    Unsolvable,
    I32AbsError,
}

/// All the Warning Types.
///
/// -  Free variables detected, result may be wrong.
/// -  No warning.
#[derive(PartialEq, Debug)]
pub enum WarnCases {
    FreeVariablesDetected,
    NoWarn,
}
