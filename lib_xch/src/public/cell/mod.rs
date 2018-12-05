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

// Overall: this is a component for safe calculations.

// mods
mod base;
mod fromstr;
mod integer;
mod num;
mod signed;

use std::cmp::Ordering;
// inside use(s)
use api::traits::{CheckedCalc, CheckedType};

#[derive(Copy, Clone, Debug)]
pub struct Cell<U> {
    error_tag: bool,
    data: U,
}

impl<U> Cell<U> {
    #[cfg(test)]
    pub fn new(data: U) -> Self {
        Cell {
            error_tag: false,
            data,
        }
    }
    pub fn get_tag(&self) -> bool {
        self.error_tag
    }
}

impl<U: Clone> Cell<U> {
    pub fn get_data(&self) -> U {
        self.data.clone()
    }
}

impl<U: CheckedType + CheckedCalc> CheckedType for Cell<U> where
    std::num::ParseIntError: std::convert::From<<U as ::num::Num>::FromStrRadixErr>
        + std::convert::From<<U as std::str::FromStr>::Err>
{}

// impls of `Eq`, `PartialEq`, `Ord`, `PartialOrd`
impl<U: PartialEq> PartialEq for Cell<U> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<U: PartialEq> Eq for Cell<U> {}

impl<U: PartialOrd + Ord> Ord for Cell<U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl<U: PartialEq + Ord> PartialOrd for Cell<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn overflow_test_1() {
        let a = Cell::<i32>::new(std::i32::MAX);
        let b = Cell::<i32>::new(1);
        let c = a + b;
        assert_eq!(c.error_tag, true);
        assert_eq!(c, Cell::<i32>::new(1));
    }

    #[test]
    fn overflow_test_2() {
        let a = Cell::<i32>::new(std::i32::MIN);
        let b = Cell::<i32>::new(1);
        let c = a - b;
        assert_eq!(c.error_tag, true);
        assert_eq!(c, Cell::<i32>::new(1));
    }

    #[test]
    fn overflow_test_3() {
        let a = Cell::<i32>::new(std::i32::MAX);
        let b = Cell::<i32>::new(2);
        let c = a * b;
        assert_eq!(c.error_tag, true);
        assert_eq!(c, Cell::<i32>::new(1));
    }

    #[test]
    fn overflow_test_4() {
        let a = Cell::<i32>::new(std::i32::MIN);
        let b = Cell::<i32>::new(-1);
        let c = a / b;
        assert_eq!(c.error_tag, true);
        assert_eq!(c, Cell::<i32>::new(1));
    }
}
