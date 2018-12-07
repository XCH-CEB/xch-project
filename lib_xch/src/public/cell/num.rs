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

use num::Num;
use std::num::ParseIntError;
// inside use(s)
use super::Cell;
use crate::api::traits::CheckedCalc;

impl<U: Num + CheckedCalc> Num for Cell<U>
where
    std::num::ParseIntError: std::convert::From<<U as num::Num>::FromStrRadixErr>,
{
    type FromStrRadixErr = ParseIntError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let data = U::from_str_radix(str, radix)?;
        Ok(Cell {
            error_tag: false,
            data,
        })
    }
}
