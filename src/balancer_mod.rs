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

use std::vec::Vec;
// inside uses
use structs::ChemicalEquation;
use api::handler::{ErrorCases, ResultHandler};
use api::handler::ErrorCases::NoAnswer;
use api::traits::CheckedType;
use public::{safe_calc, Operator};
use math_methods::gauss_eliminate::GaussianElimination;
use math_methods::frac_util::Frac;
use math_methods::basic_fns::nlcm;

pub fn xch_balancer<T: CheckedType>(
    list: &[Vec<T>],
    chmcl_f_sut: &ChemicalEquation,
) -> Result<ResultHandler<Vec<T>>, ErrorCases> {
    let mut equation_matrix: Vec<Vec<Frac<T>>> = Vec::new();
    let result_matrix =
        vec![Frac::new(T::zero(), T::one()); safe_calc(&list.len(), &1, &Operator::Sub)?];
    for item in list.iter().skip(1) {
        let mut v: Vec<Frac<T>> = Vec::new();
        for item_j in item.iter()
            .take(safe_calc(&chmcl_f_sut.left_num, &1, &Operator::Add)?)
            .skip(1)
        {
            v.push(Frac::new(*item_j, T::one()));
        }
        for item_j in item.iter()
            .take(safe_calc(&chmcl_f_sut.sum, &1, &Operator::Add)?)
            .skip(safe_calc(&chmcl_f_sut.left_num, &1, &Operator::Add)?)
        {
            v.push(Frac::new(
                safe_calc(item_j, &T::zero(), &Operator::Neg)?,
                T::one(),
            ));
        }
        equation_matrix.push(v);
    }
    let gauss_ans = GaussianElimination::<T>::new(
        equation_matrix,
        result_matrix,
        safe_calc(&list.len(), &1, &Operator::Sub)?,
        chmcl_f_sut.sum,
    ).solve()?;
    let int_set = &mut to_int_set(gauss_ans.result)?[..];
    Ok(ResultHandler {
        warn_message: gauss_ans.warn_message,
        result: int_set.to_vec(),
    })
}

fn to_int_set<T: CheckedType>(mut v: Vec<Frac<T>>) -> Result<Vec<T>, ErrorCases> {
    let mut tmp: Vec<T> = Vec::new();
    for i in &v {
        tmp.push(i.denominator);
    }
    let lcm = nlcm(tmp)?;
    let mut result: Vec<T> = Vec::new();
    for i in &mut v {
        i.numerator = safe_calc(
            &i.numerator,
            &safe_calc(&lcm, &i.denominator, &Operator::Div)?,
            &Operator::Mul,
        )?;
        if i.numerator <= T::zero() {
            return Err(NoAnswer);
        }
        result.push(i.numerator);
    }
    Ok(result)
}
