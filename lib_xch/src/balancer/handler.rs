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

// Overall: This is the source code of the AlphaForce Balancer.

use nalgebra::base::{Dynamic, MatrixMN};
use num::rational::Ratio;
use std::vec::Vec;
// inside use(s)
use super::maths::g_elim::GaussianElimination;
use crate::public::{failures::ErrorCases, traits::CheckedType, types::DataSetIn};

pub fn balancer<T: CheckedType>(ds: DataSetIn<T>) -> Result<Vec<Vec<T>>, ErrorCases> {
    let (cd, data) = ds;
    let v = data
        .iter()
        .flat_map(|x| x)
        .map(|x| Ratio::<T>::from_integer(*x))
        .collect::<Vec<_>>();
    let equation_matrix =
        MatrixMN::<Ratio<T>, Dynamic, Dynamic>::from_row_slice(data.len(), cd.sum, &v[..]);
    let ans = GaussianElimination::<T>::new(equation_matrix).solve()?;
    let result = ans
        .into_iter()
        .map(|v| {
            let lcm: T = v.iter().fold(T::one(), |lcm, ratio| lcm.lcm(ratio.denom()));
            v.into_iter()
                .map(|ratio| lcm / *ratio.denom() * *ratio.numer())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Ok(result)
}
