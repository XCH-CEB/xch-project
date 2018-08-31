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
use std::ops::{Add, AddAssign, Mul, MulAssign};
// inside uses
use api::traits::CheckedType;

#[derive(Debug)]
pub struct AtomDict<T: CheckedType> {
    dict: HashMap<String, T>,
}

impl<T: CheckedType> AtomDict<T> {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: T) {
        self.dict.insert(k, v);
    }

    pub fn get_dict(&self) -> &HashMap<String, T> {
        &self.dict
    }
}

impl<T: CheckedType> Add for AtomDict<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: CheckedType> Mul<T> for AtomDict<T> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: CheckedType> AddAssign for AtomDict<T> {
    fn add_assign(&mut self, rhs: Self) {
        rhs.dict
            .into_iter()
            .for_each(|(k, v)| *self.dict.entry(k).or_insert_with(T::zero) += v);
    }
}

impl<T: CheckedType> MulAssign<T> for AtomDict<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.dict.iter_mut().for_each(|(_, v)| *v *= rhs);
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::AtomDict;
    use std::collections::HashMap;

    #[test]
    fn add_test() {
        let mut a = AtomDict::<i32>::new(); // CH4
        let mut b = AtomDict::<i32>::new(); // H2O
        a.insert("C".to_string(), 1);
        a.insert("H".to_string(), 4);
        b.insert("H".to_string(), 2);
        b.insert("O".to_string(), 1);
        let c = a + b;
        assert_eq!(
            c.dict,
            [
                ("H".to_string(), 6),
                ("O".to_string(), 1),
                ("C".to_string(), 1)
            ].iter()
                .cloned()
                .collect::<HashMap<String, i32>>()
        );
    }

    #[test]
    fn mul_test() {
        let mut a = AtomDict::<i32>::new(); // CH4
        a.insert("C".to_string(), 1);
        a.insert("H".to_string(), 4);
        let c = a * 2;
        assert_eq!(
            c.get_dict(),
            &[("H".to_string(), 8), ("C".to_string(), 2)]
                .iter()
                .cloned()
                .collect::<HashMap<String, i32>>()
        );
    }
}
