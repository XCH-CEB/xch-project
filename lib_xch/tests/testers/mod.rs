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
use lib_xch::api::traits::{CheckedCalc, CheckedType};

pub fn tester<T: CheckedType + CheckedCalc>(equ: &str, v: &[&[T]])
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    let tmp = match handler_api::<T>(equ) {
        Ok((_, v)) => v,
        Err((e, _)) => {
            println!("{:?}", e);
            panic!(e)
        }
    };
    assert_eq!(tmp, v);
}

pub fn tester_error<T: CheckedType + CheckedCalc>(payload: &str, err: &ErrorCases)
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    if let Err((e, _)) = handler_api::<T>(payload) {
        assert_eq!(e, *err);
    } else {
        panic!("Failed!"); // `handler_api::<T>` returned `Ok(_)`
    }
}
