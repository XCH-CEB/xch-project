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
use structs::ElementStruct;
use parser_mod::get_token;

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
    elements_table: Vec<ElementStruct>, // store the index of elements
    list: Vec<Vec<i32>>,
    formula_sum: i32,
}

impl TableDesc {
    pub fn store_in_table(&mut self, formula: &String, location: usize) -> Result<bool, String> {
        for t in get_token(formula)? {
            if self.find_element_in_table(&t.token_name).is_ok() == false {
                let len = self.elements_table.len();
                self.elements_table.push(ElementStruct {
                    name: t.token_name.clone(),
                    num: len + 1, // WARN: the elements_table[0].num will be 1
                });
                self.update_list_vec();
            }

            {
                // store data in table
                let tmp = self.find_element_in_table(&t.token_name)?; // It have been checked.
                self.list[tmp][location] = match self.list[tmp][location].checked_add(t.times) {
                    Some(s) => s,
                    None => return Err("[ERROR] i32 overflow".to_string()),
                }
            }
        }
        Ok(true)
    }

    pub fn get_elements_table_len(&self) -> usize {
        self.elements_table.len()
    }

    pub fn get_list(&self) -> Vec<Vec<i32>> {
        (self.list).to_vec()
    }

    pub fn new(sum: i32) -> Self { // PLEASE call update_list_vec after new!
        Self {
            elements_table: Vec::new(),
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

    fn find_element_in_table(&self, target: &String) -> Result<usize, String> {
        for i in &(self.elements_table) {
            if i.name == *target {
                return Ok(i.num);
            }
        }
        Err("[ERROR] Not found!".to_string())
    }
}
