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

use lib_xch::public::{
    failures::ErrorCases,
    handler::Handler,
    traits::{CheckedCalc, CheckedType},
};

pub fn tester<T: CheckedType + CheckedCalc>(equ: &str, v: &[&[T]])
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    assert_eq!(
        match Handler::<T>::new(equ).handle() {
            Ok((_, v)) => v
                .iter()
                .map(|x| x.iter().map(|x| **x).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            Err(e) => panic!(e),
        },
        v
    );
}

pub fn tester_error<T: CheckedType + CheckedCalc>(payload: &str, err: &ErrorCases)
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    if let Err(e) = Handler::<T>::new(payload).handle() {
        assert_eq!(&e, err);
    } else {
        panic!("Failed!"); // `handler_api::<T>` returned `Ok(_)`
    }
}
