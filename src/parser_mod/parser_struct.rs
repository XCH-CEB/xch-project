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

use std::collections::HashMap;
// inside uses
use super::get_token;
use handler::ErrorCases;
use handler::ErrorCases::NotFound;
use public::{safe_calc, Operator};

pub struct FormulaDesc {
    pub formula_self: String,
    pub times: i32,
    pub all: String,
}

pub struct TokenDesc {
    pub token_name: String,
    pub times: i32,
}

// Object-Oriented
pub struct TableDesc {
    elements_table: HashMap<String, usize>, // store the index of elements
    list: Vec<Vec<i32>>,
    formula_sum: i32,
}

impl TableDesc {
    pub fn store_in_table(&mut self, formula: &str, location: usize) -> Result<bool, ErrorCases> {
        for t in get_token(formula)? {
            if !self.elements_table.contains_key(&t.token_name) {
                let len = self.elements_table.len();
                self.elements_table.insert(
                    t.token_name.clone(),
                    len + 1, // WARN: the elements_table[0].num will be 1
                );
                self.update_list_vec();
            }

            {
                // store data in table
                let tmp = match self.elements_table.get(&t.token_name) {
                    Some(s) => *s,
                    None => return Err(NotFound),
                }; // It have been checked.
                self.list[tmp][location] =
                    safe_calc(self.list[tmp][location], t.times, &Operator::Add)?;
            }
        }
        Ok(true)
    }

    pub fn get_list(&self) -> Vec<Vec<i32>> {
        (self.list).to_vec()
    }

    pub fn new(sum: i32) -> Self {
        // PLEASE call update_list_vec after new!
        Self {
            elements_table: HashMap::new(),
            list: Vec::new(),
            formula_sum: sum,
        }
    }

    pub fn update_list_vec(&mut self) {
        let v = self.generate_vec();
        self.list.push(v);
    }

    fn generate_vec(&self) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        for _ in 0..self.formula_sum + 1 {
            v.push(0);
        }
        v
    }
}
