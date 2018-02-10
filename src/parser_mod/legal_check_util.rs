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

pub fn legal_check(equation: &String) -> Result<bool, String> {
    let equation = equation.chars().into_iter().collect::<Vec<_>>();
    let mut tmp = 0;
    for i in equation {
        if check_char(i) == 0 {
            return Err("[ERROR] Illegal Equation!".to_string());
        }
        if i == '=' {
            tmp = tmp + 1;
        }
    }
    if tmp != 1 {
        return Err("[ERROR] Illegal Equation!".to_string());
    }
    Ok(true)
}

pub fn legal_check_brackets(formula: &String) -> Result<bool, String> {
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

fn brackets_matcher(formula: &Vec<char>, pos: usize, mode: bool) -> Result<usize, String> {
    let mut fake_stack = 0;

    if mode == true {
        for i in pos + 1..formula.len() {
            if formula[i] == '(' {
                fake_stack = fake_stack + 1;
            }
            if formula[i] == ')' {
                if fake_stack == 0 {
                    return Ok(i);
                } else {
                    fake_stack = fake_stack - 1;
                }
            }
        }
    } else {
        for i in (0..pos).rev() {
            if formula[i] == ')' {
                fake_stack = fake_stack + 1;
            }
            if formula[i] == '(' {
                if fake_stack == 0 {
                    return Ok(i);
                } else {
                    fake_stack = fake_stack - 1;
                }
            }
        }
    }
    Err("[ERROR] Can't match!".to_string())
}

fn check_char(test: char) -> i32 {
    if (test >= 'a') && (test <= 'z') {
        return 1; // 'a'~'z'
    } else if (test >= 'A') && (test <= 'Z') {
        return 2; // 'A'~'Z'
    } else if (test >= '0') && (test <= '9') {
        return 3; // '0'~'9'
    } else if (test == '(') || (test <= ')') {
        return 4; // ( or )
    } else if (test == '+') || (test == '=') {
        return 5; // + or =
    } else {
        return 0; // nothing!
    }
}
