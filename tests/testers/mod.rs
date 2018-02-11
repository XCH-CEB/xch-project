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

use lib_xch::handler::handler_api;

pub fn tester(equ: &str, range: i32, v: &[i32]) {
    let tmp = match handler_api(equ, range) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    assert_eq!(tmp, v);
}

pub fn tester_error(equ: &str, range: i32, err: &str) {
    let tmp = match handler_api(equ, range) {
        Ok(_) => panic!("Failed!"),
        Err(s) => s.error_message,
    };
    assert_eq!(tmp, err.to_string());
}
