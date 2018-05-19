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
use api::handler::ErrorCases::{IllegalEquation, IllegalUsage, MatchError};
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
        // In this case, it is only using `brackets_matcher`'s checking function.
        // What it returns doesn't matter.
        match formula[i] {
            '(' => brackets_matcher(&formula, i, true)?,
            ')' => brackets_matcher(&formula, i, false)?,
            _ => 0,
        };
    }
    Ok(true)
}

fn brackets_matcher(formula: &[char], pos: usize, mode: bool) -> Result<usize, ErrorCases> {
    let mut fake_stack = 0;
    if (mode & (formula[pos] == ')')) || ((!mode) & (formula[pos] == '(')) {
        return Err(IllegalUsage);
    }

    if mode {
        for (i, item) in formula.iter().enumerate().skip(pos + 1) {
            match *item {
                '(' => fake_stack = safe_calc(&fake_stack, &1, &Operator::Add)?,
                ')' => {
                    if fake_stack == 0 {
                        return Ok(i);
                    } else {
                        fake_stack = safe_calc(&fake_stack, &1, &Operator::Sub)?;
                    }
                }
                _ => (),
            };
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
    match test {
        'a'...'z' => 1,
        'A'...'Z' => 2,
        '0'...'9' => 3,
        '(' | ')' => 4,
        '+' | '=' => 5,
        _ => 0,
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::{brackets_matcher, legal_check_brackets};
    use api::handler::ErrorCases;

    #[test]
    fn check_brackets() {
        assert_eq!(legal_check_brackets("(())"), Ok(true));
        assert_eq!(legal_check_brackets("(()))"), Err(ErrorCases::MatchError));
    }

    #[test]
    fn match_brackets() {
        assert_eq!(
            brackets_matcher(&['(', '(', '(', ')', ')', ')'], 2, false),
            Err(ErrorCases::IllegalUsage)
        );
        assert_eq!(
            brackets_matcher(&['(', '(', '(', ')', ')', ')'], 3, true),
            Err(ErrorCases::IllegalUsage)
        );
        assert_eq!(
            brackets_matcher(&['(', '(', '(', ')', ')', ')'], 2, true),
            Ok(3)
        );
        assert_eq!(
            brackets_matcher(&['(', '(', '('], 2, true),
            Err(ErrorCases::MatchError)
        );
    }
}
