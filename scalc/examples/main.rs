// Copyright 2019 LEXUGE
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

use scalc::SCell;

fn main() -> Result<(), String> {
    let a = SCell::<i32>::new(12) * SCell::<i32>::new(3);
    assert_eq!(*a.ok_or("overflow")?.get_data(), 36);

    // Addition will result in `None` in the presence of overflow behavior(s)
    let a = SCell::<i32>::new(std::i32::MAX) + SCell::<i32>::new(1);
    assert_eq!(a.is_none(), true);
    Ok(())
}
