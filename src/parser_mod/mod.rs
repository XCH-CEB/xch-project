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
// inside uses
use self::legal_check_util::{legal_check, legal_check_brackets};
use self::parser_struct::{FormulaDesc, TableDesc, TokenDesc};
use api::handler::ErrorCases;
use api::handler::ErrorCases::{NoTokens, ParseError, SplitError};
use api::traits::CheckedType;
use public::{safe_calc, Operator};
use structs::ChemicalEquation;

pub fn xch_parser<T: CheckedType>(
    equation: &str,
) -> Result<(ChemicalEquation, Vec<Vec<T>>), ErrorCases> {
    legal_check(equation)?;
    let mut chemical_equation_struct = ChemicalEquation {
        left_num: 0,
        right_num: 0,
        sum: 0,
    };
    {
        // block to get chemical_equation_struct.sum
        let v: Vec<&str> = equation.split('=').collect();
        let equation_left: String = String::from(v[0]);
        let equation_right: String = String::from(v[1]);
        let tmp1 = parser_get_sum(&equation_left)?;
        let tmp2 = parser_get_sum(&equation_right)?;
        chemical_equation_struct.sum = safe_calc(&tmp1, &tmp2, &Operator::Add)?;
    }
    let mut table = TableDesc::new(chemical_equation_struct.sum);
    table.update_list_vec(); // first access will be like list[1][1]

    {
        // block to call parsers
        let v: Vec<&str> = equation.split('=').collect();
        let equation_left: String = String::from(v[0]);
        let equation_right: String = String::from(v[1]);

        chemical_equation_struct.left_num = part_parser(&equation_left, &mut table, 0)?;
        chemical_equation_struct.right_num = part_parser(
            &equation_right,
            &mut table,
            chemical_equation_struct.left_num,
        )?;
    }

    // return
    Ok((chemical_equation_struct, table.get_list()))
}

fn parser_get_sum(equation: &str) -> Result<usize, ErrorCases> {
    let mut sum: usize = 0;
    for _ in equation.split('+') {
        sum = safe_calc(&sum, &1, &Operator::Add)?;
    }
    Ok(sum)
}

fn part_parser<T: CheckedType>(
    equation: &str,
    table: &mut TableDesc<T>,
    begin: usize,
) -> Result<usize, ErrorCases> {
    let mut sum = begin;
    for formula in equation.split('+') {
        sum = safe_calc(&sum, &1, &Operator::Add)?;
        legal_check_brackets(&formula.to_string())?;
        parser_formula(&formula.to_string(), table, sum)?;
    }
    Ok(sum - begin)
}

fn formula_spliter<T: CheckedType>(target: &str) -> Result<Vec<FormulaDesc<T>>, ErrorCases> {
    let mut v: Vec<FormulaDesc<T>> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((([A-Z][a-z]*(\d+)*)+)\)(\d+)*").unwrap(); // safe unwrap
    }

    if !RE.is_match(target) {
        return Err(SplitError);
    }
    for cap in RE.captures_iter(target) {
        let mut times: T;
        let cap4 = cap.get(4).map_or("", |m| m.as_str());
        if cap4 == "" {
            times = T::one();
        } else {
            times = match cap4.trim().parse::<T>() {
                Ok(s) => s,
                Err(_) => return Err(ParseError),
            }
        }
        v.push(FormulaDesc {
            formula_self: cap[1].to_string(),
            times,
            all: cap[0].to_string(),
        });
    }
    Ok(v)
}

fn get_token<T: CheckedType>(target: &str) -> Result<Vec<TokenDesc<T>>, ErrorCases> {
    let mut v: Vec<TokenDesc<T>> = Vec::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Z][a-z]*)(\d+)*").unwrap(); // safe unwrap
    }
    if !RE.is_match(target) {
        return Err(NoTokens);
    }
    for cap in RE.captures_iter(target) {
        let cap2 = cap.get(2).map_or("", |m| m.as_str());
        let mut times: T;
        if cap2 == "" {
            times = T::one();
        } else {
            times = match cap2.trim().parse::<T>() {
                Ok(s) => s,
                Err(_) => return Err(ParseError),
            }
        }
        v.push(TokenDesc {
            token_name: cap[1].to_string(),
            times,
        });
    }
    Ok(v)
}

fn mul_phrase<T: CheckedType>(phrase: &FormulaDesc<T>) -> Result<String, ErrorCases> {
    let mut v = get_token(&phrase.formula_self)?;
    for token in &mut v {
        token.times = safe_calc(&token.times, &phrase.times, &Operator::Mul)?;
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

fn parser_formula<T: CheckedType>(
    // parse the chemical formula
    formula: &str,
    table: &mut TableDesc<T>,
    location: usize,
) -> Result<bool, ErrorCases> {
    let formula_backup = formula;
    let mut formula = format!("({})", formula_backup);

    formula_spliter::<T>(&formula)?;
    while formula_spliter::<T>(&formula).is_ok() {
        for p in formula_spliter::<T>(&formula)? {
            formula = replace_phrase(&formula, &p.all, &(mul_phrase(&p)?));
        }
    }
    table.store_in_table(&formula, location)?;
    Ok(true)
}
