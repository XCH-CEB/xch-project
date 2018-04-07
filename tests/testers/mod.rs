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

use lib_xch::api::handler::{handler_api, ErrorCases};
use lib_xch::api::traits::CheckedType;

pub fn tester<T: CheckedType>(equ: &str, v: &[T]) {
    let tmp = match handler_api::<T>(equ) {
        Ok(v) => v.result,
        Err(e) => {
            println!("{:?}", e.error_message);
            panic!(e.error_message)
        }
    };
    assert_eq!(tmp, v);
}

pub fn tester_error<T: CheckedType>(payload: &str, err: &ErrorCases) {
    let tmp = match handler_api::<T>(payload) {
        Ok(_) => panic!("Failed!"),
        Err(s) => s.error_message,
    };
    assert_eq!(tmp, *err);
}
