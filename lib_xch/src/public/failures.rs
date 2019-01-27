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

//! Failures which exposes to the public

use failure::Fail;

/// All the Error Types.
#[derive(PartialEq, Fail, Debug)]
pub enum ErrorCases {
    /// Overflow.
    #[fail(display = "Overflow occured during calculation")]
    Overflow,
    /// Parser's error with a message.
    #[fail(display = "{}", _0)]
    ParserError(String),
    /// Only the [zero solution](http://www.mathwords.com/t/trivial.htm) can be found.
    #[fail(display = "AlphaForce can only find trivial solution")]
    ZeroSolution,
}
