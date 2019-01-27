// Copyright 2017-2019 LEXUGE
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

use num::Integer;
// inside use(s)
use super::{
    super::traits::{CheckedCalc, CheckedType},
    Cell,
};

impl<U: CheckedType + CheckedCalc> Integer for Cell<U>
where
    std::num::ParseIntError: std::convert::From<<U as num::Num>::FromStrRadixErr>,
{
    fn div_floor(&self, other: &Self) -> Self {
        Cell {
            error_tag: self.error_tag | other.error_tag,
            data: self.data.div_floor(&other.data),
        }
    }
    fn mod_floor(&self, other: &Self) -> Self {
        Cell {
            error_tag: self.error_tag | other.error_tag,
            data: self.data.mod_floor(&other.data),
        }
    }
    fn gcd(&self, other: &Self) -> Self {
        Cell {
            error_tag: self.error_tag | other.error_tag,
            data: self.data.gcd(&other.data),
        }
    }
    fn lcm(&self, other: &Self) -> Self {
        Cell {
            error_tag: self.error_tag | other.error_tag,
            data: self.data.lcm(&other.data),
        }
    }
    fn divides(&self, other: &Self) -> bool {
        self.data.divides(&other.data)
    }
    fn is_multiple_of(&self, other: &Self) -> bool {
        self.data.is_multiple_of(&other.data)
    }
    fn is_even(&self) -> bool {
        self.data.is_even()
    }
    fn is_odd(&self) -> bool {
        self.data.is_odd()
    }
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        let (quotient, remainder) = self.data.div_rem(&other.data);
        (
            Cell {
                error_tag: self.error_tag | other.error_tag,
                data: quotient,
            },
            Cell {
                error_tag: self.error_tag | other.error_tag,
                data: remainder,
            },
        )
    }
}
