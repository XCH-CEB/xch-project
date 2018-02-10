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
mod parser_struct;

use regex::Regex;
use std::string::String;
use std::vec::Vec;
use std::i32;
// inside uses
use structs::ChemicalEquation;
use self::parser_struct::{TableDesc,FormulaDesc,TokenDesc};
use self::legal_check_util::{legal_check, legal_check_brackets};

pub fn xch_parser(equation: String) -> Result<(ChemicalEquation, usize, Vec<Vec<i32>>),String> {
    legal_check(&equation)?;
    let mut chemical_equation_struct = ChemicalEquation {
        left_num: 0,
        right_num: 0,
        sum: 0,
    };
    { // block to get chemical_equation_struct.sum
        let v: Vec<&str> = equation.split('=').collect();
        let equation_left: String = String::from(v[0]);
        let equation_right: String = String::from(v[1]);
        chemical_equation_struct.sum =
            parser_get_sum(&equation_left)? + parser_get_sum(&equation_right)?;
    }
    let mut table = TableDesc::new(chemical_equation_struct.sum);
    table.update_list_vec(); // first access will be like list[1][1]

    { // block to call parsers
        let v: Vec<&str> = equation.split('=').collect();
        let equation_left: String = String::from(v[0]);
        let equation_right: String = String::from(v[1]);

        chemical_equation_struct.left_num = part_parser(
            &equation_left,
            &mut table,
            0,
        )?;
        chemical_equation_struct.right_num = part_parser(
            &equation_right,
            &mut table,
            chemical_equation_struct.left_num,
        )?;
    }

    // return
    Ok((chemical_equation_struct, table.get_elements_table_len(), table.get_list()))
}

fn parser_get_sum(equation: &String) -> Result<i32,String> {
    let mut sum: i32 = 0;
    for _ in equation.split('+') {
        sum = match sum.checked_add(1) {
            Some(s) => s,
            None => return Err("[ERROR] i32 overflow: Illegal Equation!".to_string()),
        }
    }
    Ok(sum)
}

fn part_parser(
    equation: &String,
    table: &mut TableDesc,
    begin: i32,
) -> Result<i32,String> {
    let mut sum = begin;
    for formula in equation.split('+') {
        sum = sum + 1;
        legal_check_brackets(&formula.to_string())?;
        parser_formula(
            &formula.to_string(),
            table,
            sum as usize,
        )?;
    }
    Ok(sum - begin)
}

fn formula_spliter(target: &str) -> Result<Vec<FormulaDesc>, String> {
    let mut v: Vec<FormulaDesc> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((([A-Z][a-z]*(\d+)*)+)\)(\d+)*").unwrap();
    }

    if RE.is_match(target) == false {
        return Err("[ERROR] No more to split!".to_string());
    }
    for cap in RE.captures_iter(target) {
        let mut times: i32;
        let cap4 = cap.get(4).map_or("", |m| m.as_str());
        if cap4 == "" {
            times = 1;
        } else {
            times = match cap4.trim().parse::<i32>() {
                Ok(s) => s,
                Err(_) => return Err("[ERROR] Not a number!".to_string()),
            }
        }
        v.push(FormulaDesc {
            formula_self: cap[1].to_string(),
            times: times,
            all: cap[0].to_string(),
        });
    }
    Ok(v)
}

fn get_token(target: &str) -> Result<Vec<TokenDesc>,String> {
    let mut v: Vec<TokenDesc> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Z][a-z]*)(\d+)*").unwrap();
    }
    if RE.is_match(target) == false {
        return Err("[ERROR] No tokens!".to_string());
    }
    for cap in RE.captures_iter(target) {
        let cap2 = cap.get(2).map_or("", |m| m.as_str());
        let mut times: i32;
        if cap2 == "" {
            times = 1;
        } else {
            times = match cap2.trim().parse::<i32>() {
                Ok(s) => s,
                Err(_) => return Err("[ERROR] Not a number!".to_string()),
            }
        }
        v.push(TokenDesc {
            token_name: cap[1].to_string(),
            times: times,
        });
    }
    Ok(v)
}

fn mul_phrase(phrase: &FormulaDesc) -> Result<String,String> {
    let mut v = get_token(&phrase.formula_self)?;
    for token in &mut v {
        token.times = match token.times.checked_mul(phrase.times) {
            Some(s) => s,
            None => return Err("[ERROR] i32 overflow".to_string()),
        }
    }
    let mut s: String = String::new();
    for token in v {
        s = s + &token.token_name;
        s = s + &token.times.to_string();
    }
    Ok(s)
}

fn replace_phrase(target: &str, src: &str, des: &str) -> String {
    str::replace(target, src, des)
}

fn parser_formula(
    // parse the chemical formula
    formula: &String,
    table: &mut TableDesc,
    location: usize,
) -> Result<bool, String>{
    let formula_backup = formula;
    let mut formula = format!("({})", formula_backup);

    while formula_spliter(&formula).is_ok() == true {
        for p in formula_spliter(&formula)? {
            formula = replace_phrase(&formula, &p.all, &(mul_phrase(&p)?));
        }
    }
    table.store_in_table(&formula,location)?;
    Ok(true)
}
