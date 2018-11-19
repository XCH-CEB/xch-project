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
use super::atomdict::AtomDict;
use api::{handler::ErrorCases, traits::CheckedType};

// This is the data structure of describing the result of Delta-3 Parser.
// This is the form of the `list`:
// |     \     | formula_1 | formula_2 | ...       | formula_n |
// | element_1 | ...       | ...       | ...       | ...       |
// | element_2 | ...       | ...       | ...       | ...       |
// | ...       | ...       | ...       | ...       | ...       |
// | element_n | ...       | ...       | ...       | ...       |
pub struct TableDesc<T: CheckedType> {
    elements_table: HashMap<String, usize>, // store the index of elements
    list: Vec<Vec<T>>,
    formula_sum: usize,
}

impl<T: CheckedType> TableDesc<T> {
    pub fn store_in_table(
        &mut self,
        atomdict: &AtomDict<T>,
        location: usize,
        neg: bool,
    ) -> Result<(), ErrorCases> {
        for (k, v) in atomdict.get_dict().iter() {
            if !self.elements_table.contains_key(k) {
                let len = self.elements_table.len();
                self.elements_table.insert(k.to_string(), len);
                self.list.push(generate_vec(self.formula_sum));
            }
            // store data in table
            let value = if neg { -(*v) } else { *v };
            self.list[self.elements_table[k]][location] += value;
        }
        Ok(())
    }

    pub fn get_list(&self) -> Vec<Vec<T>> {
        (self.list).to_vec()
    }

    pub fn new(formula_sum: usize) -> Self {
        Self {
            elements_table: HashMap::new(),
            list: Vec::new(),
            formula_sum,
        }
    }
}

fn generate_vec<T: CheckedType>(capacity: usize) -> Vec<T> {
    vec![T::zero(); capacity]
}
