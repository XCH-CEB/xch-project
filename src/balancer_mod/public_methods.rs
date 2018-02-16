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

// Overall: This is the source code of the AlphaForce Balancer.

use handler::ErrorCases::I32Overflow;
use handler::ErrorCases;

pub fn gcd(mut a: i32, mut b: i32) -> Result<i32, ErrorCases> {
    let mut t: i32;
    while b != 0 {
        t = b;
        b = match a.checked_rem(b) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        a = t;
    }
    Ok(a)
}

pub fn lcm(a: i32, b: i32) -> Result<i32, ErrorCases> {
    let a_b = match a.checked_mul(b) {
        Some(s) => s,
        None => return Err(I32Overflow),
    };
    match a_b.checked_div(gcd(a, b)?) {
        Some(s) => Ok(s),
        None => Err(I32Overflow),
    }
}

pub fn nlcm(v: Vec<i32>) -> Result<i32, ErrorCases> {
    let mut ans: i32 = 1;
    for i in v {
        ans = lcm(ans, i)?;
    }
    Ok(ans)
}
