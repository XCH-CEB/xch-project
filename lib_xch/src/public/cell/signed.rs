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

use num::Signed;
// inside use(s)
use super::{super::traits::CheckedCalc, Cell};

impl<U: Signed + CheckedCalc> Signed for Cell<U>
where
    std::num::ParseIntError: std::convert::From<<U as num::Num>::FromStrRadixErr>,
{
    fn abs(&self) -> Self {
        Cell {
            error_tag: self.error_tag,
            data: self.data.abs(),
        }
    }
    fn abs_sub(&self, other: &Self) -> Self {
        Cell {
            error_tag: self.error_tag | other.error_tag,
            data: self.data.abs_sub(&other.data),
        }
    }
    fn signum(&self) -> Self {
        Cell {
            error_tag: self.error_tag,
            data: self.data.signum(),
        }
    }
    fn is_positive(&self) -> bool {
        self.data.is_positive()
    }
    fn is_negative(&self) -> bool {
        self.data.is_negative()
    }
}
