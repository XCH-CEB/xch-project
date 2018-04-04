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

use handler::ErrorCases;
use public::{safe_calc, Operator};

pub fn gcd(mut a: i32, mut b: i32) -> Result<i32, ErrorCases> {
    let mut t: i32;
    while b != 0 {
        t = b;
        b = safe_calc(a, b, &Operator::Rem)?;
        a = t;
    }
    Ok(a)
}

pub fn lcm(a: i32, b: i32) -> Result<i32, ErrorCases> {
    let a_b = safe_calc(a, b, &Operator::Mul)?;
    safe_calc(a_b, gcd(a, b)?, &Operator::Div)
}

pub fn nlcm(v: Vec<i32>) -> Result<i32, ErrorCases> {
    let mut ans: i32 = 1;
    for i in v {
        ans = lcm(ans, i)?;
    }
    Ok(ans)
}
