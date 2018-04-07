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

// Overall: This is the source code of the Delta-3 Parser.

// inside uses
use api::handler::ErrorCases;
use api::handler::ErrorCases::{IllegalEquation, MatchError};
use public::{safe_calc, Operator};

pub fn legal_check(equation: &str) -> Result<bool, ErrorCases> {
    let equation = equation.chars().into_iter().collect::<Vec<_>>();
    let mut tmp = 0;
    for i in equation {
        if check_char(i) == 0 {
            return Err(IllegalEquation);
        }
        if i == '=' {
            tmp = safe_calc(&tmp, &1, &Operator::Add)?;
        }
    }
    if tmp != 1 {
        return Err(IllegalEquation);
    }
    Ok(true)
}

pub fn legal_check_brackets(formula: &str) -> Result<bool, ErrorCases> {
    let formula = formula.chars().into_iter().collect::<Vec<_>>();
    for i in 0..formula.len() {
        if formula[i] == '(' {
            brackets_matcher(&formula, i, true)?;
        }
        if formula[i] == ')' {
            brackets_matcher(&formula, i, false)?;
        }
    }
    Ok(true)
}

fn brackets_matcher(formula: &[char], pos: usize, mode: bool) -> Result<usize, ErrorCases> {
    let mut fake_stack = 0;

    if mode {
        for (i, item) in formula.iter().enumerate().skip(pos + 1) {
            if *item == '(' {
                fake_stack = safe_calc(&fake_stack, &1, &Operator::Add)?;
            }
            if *item == ')' {
                if fake_stack == 0 {
                    return Ok(i);
                } else {
                    fake_stack = safe_calc(&fake_stack, &1, &Operator::Sub)?;
                }
            }
        }
    } else {
        for i in (0..pos).rev() {
            if formula[i] == ')' {
                fake_stack = safe_calc(&fake_stack, &1, &Operator::Add)?;
            }
            if formula[i] == '(' {
                if fake_stack == 0 {
                    return Ok(i);
                } else {
                    fake_stack = safe_calc(&fake_stack, &1, &Operator::Sub)?;
                }
            }
        }
    }
    Err(MatchError)
}

fn check_char(test: char) -> i32 {
    if (test >= 'a') && (test <= 'z') {
        1 // 'a'~'z'
    } else if (test >= 'A') && (test <= 'Z') {
        2 // 'A'~'Z'
    } else if (test >= '0') && (test <= '9') {
        3 // '0'~'9'
    } else if (test == '(') || (test <= ')') {
        4 // ( or )
    } else if (test == '+') || (test == '=') {
        5 // + or =
    } else {
        0 // nothing!
    }
}
