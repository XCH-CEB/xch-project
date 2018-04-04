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

mod gauss_eliminate;
mod math_methods;
mod frac_util;

use std::vec::Vec;
use std::i32;
// inside uses
use structs::ChemicalEquation;
use handler::{ErrorCases, ResultHandler};
use handler::ErrorCases::NoAnswer;
use self::gauss_eliminate::GaussianElimination;
use self::frac_util::Frac;
use self::math_methods::nlcm;
use public::{safe_calc, Operator};

pub fn xch_balancer(
    list: &[Vec<i32>],
    chmcl_f_sut: &ChemicalEquation,
) -> Result<ResultHandler<Vec<i32>>, ErrorCases> {
    let mut equation_matrix: Vec<Vec<Frac>> = Vec::new();
    let result_matrix = vec![Frac::new(0, 1); list.len() - 1];
    for item in list.iter().skip(1) {
        let mut v: Vec<Frac> = Vec::new();
        for item_j in item.iter()
            .take((chmcl_f_sut.left_num + 1) as usize)
            .skip(1)
        {
            v.push(Frac::new(*item_j, 1));
        }
        for item_j in item.iter()
            .take((chmcl_f_sut.sum + 1) as usize)
            .skip((chmcl_f_sut.left_num + 1) as usize)
        {
            v.push(Frac::new(-item_j, 1));
        }
        equation_matrix.push(v);
    }
    let gauss_ans = GaussianElimination::new(
        equation_matrix,
        result_matrix,
        list.len() - 1,
        chmcl_f_sut.sum as usize,
    ).solve()?;
    let int_set = &mut to_int_set(gauss_ans.result)?[..];
    Ok(ResultHandler {
        warn_message: gauss_ans.warn_message,
        result: int_set.to_vec(),
    })
}

fn to_int_set(mut v: Vec<Frac>) -> Result<Vec<i32>, ErrorCases> {
    let mut tmp: Vec<i32> = Vec::new();
    for i in &v {
        tmp.push(i.denominator);
    }
    let lcm = nlcm(tmp)?;
    let mut result: Vec<i32> = Vec::new();
    for i in &mut v {
        i.numerator *= safe_calc(lcm, i.denominator, &Operator::Div)?;
        if i.numerator <= 0 {
            return Err(NoAnswer);
        }
        result.push(i.numerator);
    }
    Ok(result)
}
