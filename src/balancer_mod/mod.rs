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

mod guass_eliminate;
mod public_methods;
mod frac_util;

use std::vec::Vec;
use std::i32;
// inside uses
use structs::ChemicalEquation;
use handler::ErrorCases;
use handler::ErrorCases::{I32Overflow, NoAnswer};
use self::guass_eliminate::GuassianElimination;
use self::frac_util::Frac;
use self::public_methods::nlcm;

pub fn xch_balancer(
    list: &[Vec<i32>],
    chmcl_f_sut: &ChemicalEquation,
) -> Result<Vec<i32>, ErrorCases> {
    let free_variable_n = if list.len() - 1 > (chmcl_f_sut.sum) as usize {
        list.len() - 1 - (chmcl_f_sut.sum) as usize
    } else {
        chmcl_f_sut.sum as usize - (list.len() - 1)
    };
    let mut equation_matrix: Vec<Vec<Frac>> = Vec::new();
    let mut result_matrix = vec![Frac::new(0, 1); list.len()];
    equation_matrix.push(vec![Frac::new(0, 1)]); // Just fill one is enough.
    for i in 1..list.len() {
        let mut v: Vec<Frac> = Vec::new();
        v.push(Frac::new(0, 1));
        for j in 1..(chmcl_f_sut.left_num + 1) as usize {
            if j <= free_variable_n {
                result_matrix[i] = result_matrix[i].sub(Frac::new(list[i][j], 1))?;
            } else {
                v.push(Frac::new(list[i][j], 1));
            }
        }
        for j in (chmcl_f_sut.left_num + 1) as usize..(chmcl_f_sut.sum + 1) as usize {
            if j <= free_variable_n {
                result_matrix[i] = result_matrix[i].add(Frac::new(list[i][j], 1))?;
            } else {
                v.push(Frac::new(-list[i][j], 1));
            }
        }
        equation_matrix.push(v);
    }
    let mut guass_ans = GuassianElimination::new(
        equation_matrix,
        result_matrix,
        chmcl_f_sut.sum as usize - free_variable_n,
    ).solve()?;
    {
        // to push free variables
        for _ in 0..free_variable_n {
            guass_ans.push(Frac::new(1, 1));
        }
    }
    let int_set = &mut to_int_set(guass_ans)?[..];
    int_set.reverse();
    Ok(int_set.to_vec())
}

fn to_int_set(mut v: Vec<Frac>) -> Result<Vec<i32>, ErrorCases> {
    let mut tmp: Vec<i32> = Vec::new();
    for i in &v {
        tmp.push(i.denominator);
    }
    let lcm = nlcm(tmp)?;
    let mut result: Vec<i32> = Vec::new();
    for i in &mut v {
        i.numerator *= match lcm.checked_div(i.denominator) {
            Some(s) => s,
            None => return Err(I32Overflow),
        };
        if i.numerator == 0 {
            return Err(NoAnswer);
        }
        result.push(i.numerator);
    }
    Ok(result)
}
