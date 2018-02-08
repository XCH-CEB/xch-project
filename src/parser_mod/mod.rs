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

// mods
mod legal_check_util;

use regex::Regex;
use std::string::String;
use std::vec::Vec;
use std::i32;
// inside uses
use structs::{ChemicalEquation, ElementStruct};
use self::legal_check_util::{legal_check, legal_check_brackets};

struct FormulaDesc {
    formula_self: String,
    times: i32,
    all: String,
}

struct TokenDesc {
    token_name: String,
    times: i32,
}

pub fn xch_parser(equation: String) -> (ChemicalEquation, Vec<ElementStruct>, Vec<Vec<i32>>) {
    legal_check(&equation);
    let mut chemical_equation_struct = ChemicalEquation {
        left_num: 0,
        right_num: 0,
        sum: 0,
    };
    let mut elements_table: Vec<ElementStruct> = Vec::new(); // store the index of elements
    let mut list: Vec<Vec<i32>> = Vec::new();
    // Unicode slice safe

    // block to call parsers
    {
        let v: Vec<&str> = equation.split('=').collect();
        let equation_left: String = String::from(v[0]);
        let equation_right: String = String::from(v[1]);

        chemical_equation_struct.sum =
            parser_get_sum(&equation_left) + parser_get_sum(&equation_right);
        list.push(generate_n_vec(chemical_equation_struct.sum + 1)); // first access will be like list[1][1]
        chemical_equation_struct.left_num = part_parser(
            &equation_left,
            &mut elements_table,
            &mut list,
            0,
            chemical_equation_struct.sum,
        );
        chemical_equation_struct.right_num = part_parser(
            &equation_right,
            &mut elements_table,
            &mut list,
            chemical_equation_struct.left_num,
            chemical_equation_struct.sum,
        );
    }

    // return
    (chemical_equation_struct, elements_table, list)
}

fn parser_get_sum(equation: &String) -> i32 {
    let mut sum: i32 = 0;
    for _ in equation.split('+') {
        sum = sum.checked_add(1)
            .expect("[ERROR] i32 overflow: Illegal Equation!");
    }
    sum
}

fn part_parser(
    equation: &String,
    elements_table: &mut Vec<ElementStruct>,
    list: &mut Vec<Vec<i32>>,
    begin: i32,
    equation_sum: i32,
) -> i32 {
    let mut sum = begin;
    for formula in equation.split('+') {
        sum = sum + 1;
        legal_check_brackets(&formula.to_string());
        parser_formula(
            &formula.to_string(),
            elements_table,
            sum as usize,
            list,
            equation_sum,
        );
    }
    sum - begin
}

fn generate_n_vec(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        v.push(0);
    }
    v
}

fn formula_spliter(target: &str) -> Result<Vec<FormulaDesc>, String> {
    let mut v: Vec<FormulaDesc> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((([A-Z][a-z]*(\d+)*)+)\)(\d+)*").unwrap();
    }

    if RE.is_match(target) == false {
        let x: Result<Vec<FormulaDesc>, String> = Err("[ERROR] No more to split!".to_string());
        return x;
    }
    for cap in RE.captures_iter(target) {
        let mut times: i32;
        let cap4 = cap.get(4).map_or("", |m| m.as_str());
        if cap4 == "" {
            times = 1;
        } else {
            times = cap4.trim().parse::<i32>().expect("[ERROR] Not a number!");
        }
        v.push(FormulaDesc {
            formula_self: cap[1].to_string(),
            times: times,
            all: cap[0].to_string(),
        });
    }
    let x: Result<Vec<FormulaDesc>, String> = Ok(v);
    x
}

fn get_token(target: &str) -> Vec<TokenDesc> {
    let mut v: Vec<TokenDesc> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Z][a-z]*)(\d+)*").unwrap();
    }
    if RE.is_match(target) == false {
        panic!("[ERROR] No tokens!");
    }
    for cap in RE.captures_iter(target) {
        let cap2 = cap.get(2).map_or("", |m| m.as_str());
        let mut times: i32;
        if cap2 == "" {
            times = 1;
        } else {
            times = cap2.trim().parse::<i32>().expect("[ERROR] Not a number!");
        }
        v.push(TokenDesc {
            token_name: cap[1].to_string(),
            times: times,
        });
    }
    v
}

fn mul_phrase(phrase: &FormulaDesc) -> String {
    let mut v = get_token(&phrase.formula_self);
    for token in &mut v {
        token.times = token
            .times
            .checked_mul(phrase.times)
            .expect("[ERROR] i32 overflow");
    }
    let mut s: String = String::new();
    for token in v {
        s = s + &token.token_name;
        s = s + &token.times.to_string();
    }
    s
}

fn replace_phrase(target: &str, src: &str, des: &str) -> String {
    str::replace(target, src, des)
}

fn store_in_table(
    formula: &String,
    elements_table: &mut Vec<ElementStruct>,
    location: usize,
    list: &mut Vec<Vec<i32>>,
    formula_sum: i32,
) {
    for t in get_token(formula) {
        if find_element_in_table(&t.token_name, elements_table).is_ok() == false {
            let len = elements_table.len();
            elements_table.push(ElementStruct {
                name: t.token_name.clone(),
                num: len + 1, // WARN: the elements_table[0].num will be 1
            });
            list.push(generate_n_vec(formula_sum + 1));
        }

        {
            // store data in table
            let tmp = find_element_in_table(&t.token_name, elements_table).unwrap();
            list[tmp][location] = list[tmp][location]
                .checked_add(t.times)
                .expect("[ERROR] i32 overflow");
        }
    }
}

fn parser_formula(
    // parse the chemical formula
    formula: &String,
    elements_table: &mut Vec<ElementStruct>,
    location: usize,
    list: &mut Vec<Vec<i32>>,
    formula_sum: i32,
) {
    let formula_backup = formula;
    let mut formula = format!("({})", formula_backup);

    while formula_spliter(&formula).is_ok() == true {
        for p in formula_spliter(&formula).unwrap() {
            formula = replace_phrase(&formula, &p.all, &mul_phrase(&p));
        }
    }
    store_in_table(&formula, elements_table, location, list, formula_sum);
}

fn find_element_in_table(target: &String, e_t: &mut Vec<ElementStruct>) -> Result<usize, String> {
    for i in e_t {
        if i.name == *target {
            let x: Result<usize, String> = Ok(i.num);
            return x;
        }
    }
    let x: Result<usize, String> = Err("[ERROR] Not found!".to_string());
    x
}
