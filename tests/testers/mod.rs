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

use lib_xch::handler::{handler_api, ErrorCases};

pub fn tester(equ: &str, v: &[i32]) {
    let tmp = match handler_api(equ) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    assert_eq!(tmp, v);
}

pub fn tester_error(payload: &str, err: &ErrorCases) {
    let tmp = match handler_api(payload) {
        Ok(_) => panic!("Failed!"),
        Err(s) => s.error_message,
    };
    assert_eq!(tmp, *err);
}
