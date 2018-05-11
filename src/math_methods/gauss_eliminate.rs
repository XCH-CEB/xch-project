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

// Overall: This is the source code of the Hyper Mathlib.

use std::collections::HashMap;
// inside uses
use super::frac_util::Frac;
use api::handler::ErrorCases::Unsolvable;
use api::handler::WarnCases::{FreeVariablesDetected, NoWarn};
use api::handler::{ErrorCases, ResultHandler};
use api::traits::CheckedType;

pub struct GaussianElimination<T: CheckedType> {
    matrix_a: Vec<Vec<Frac<T>>>, // A n*n matrix.
    matrix_b: Vec<Frac<T>>,      // A n*1 matrix.
    n: usize,
    m: usize,
}

impl<T: CheckedType> GaussianElimination<T> {
    pub fn new(matrix_a: Vec<Vec<Frac<T>>>, matrix_b: Vec<Frac<T>>, n: usize, m: usize) -> Self {
        // Create a GaussianElimination Solution.
        Self {
            matrix_a,
            matrix_b,
            n,
            m,
        }
    }

    pub fn solve(&mut self) -> Result<ResultHandler<Vec<Frac<T>>>, ErrorCases> {
        // The Gaussian-Jordan Algorithm
        for i in 0..self.n {
            let leftmosti = match self.get_leftmost_row(i) {
                Some(s) => s,
                None => continue,
            };
            self.matrix_a.swap(i, leftmosti);
            self.matrix_b.swap(i, leftmosti);
            let j = match self.get_pivot(i) {
                // if left most has no pivot, just continue.
                Some(s) => s,
                None => continue,
            };
            let maxi = self.get_max_abs_row(i, j)?;
            if self.matrix_a[maxi][j].numerator != T::zero() {
                self.matrix_a.swap(i, maxi);
                self.matrix_b.swap(i, maxi); // swap row i and maxi in matrix_a and matrix_b
                {
                    let tmp = self.matrix_a[i][j];
                    self.divide_row(i, tmp)?;
                }
                for u in i + 1..self.n {
                    let v = self.mul_row(i, self.matrix_a[u][j])?; // v has n+1 elements
                    for (k, item) in v.iter().enumerate().take(self.m) {
                        self.matrix_a[u][k] = (self.matrix_a[u][k] - (*item))?; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                    }
                    self.matrix_b[u] = (self.matrix_b[u] - v[self.m])?;
                }
            }
        } // REF

        for i in (0..self.n).rev() {
            let j = match self.get_pivot(i) {
                Some(s) => s,
                None => continue,
            };
            for u in (0..i).rev() {
                // j above i
                let v = self.mul_row(i, self.matrix_a[u][j])?; // v has n+1 elements
                for (k, item) in v.iter().enumerate().take(self.m) {
                    self.matrix_a[u][k] = (self.matrix_a[u][k] - (*item))?; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                }
                self.matrix_b[u] = (self.matrix_b[u] - v[self.m])?;
            }
        } // RREF
        let mut ans: Vec<Frac<T>> = vec![Frac::new(T::zero(), T::one()); self.m];
        let pivots = self.check()?;
        let mut free_variable = false;
        for i in (0..self.m).rev() {
            if pivots.contains_key(&i) {
                let mut sum = Frac::new(T::zero(), T::one());
                for (k, item) in ans.iter().enumerate().take(self.m).skip(i + 1) {
                    sum = (sum + (self.matrix_a[pivots[&i]][k] * (*item))?)?;
                }
                ans[i] = ((self.matrix_b[pivots[&i]] - sum)? / self.matrix_a[pivots[&i]][i])?;
            } else {
                free_variable = true;
                ans[i] = Frac::new(T::one(), T::one()); // set all free variables = 1/1.
            }
        }
        Ok(ResultHandler {
            warn_message: if free_variable {
                FreeVariablesDetected
            } else {
                NoWarn
            },
            result: ans,
        }) // x_{n} to x_{1}
    }

    fn check(&self) -> Result<HashMap<usize, usize>, ErrorCases> {
        let mut pivots: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.n {
            if self.get_pivot(i).is_some() {
                pivots.insert(self.get_pivot(i).unwrap(), i); // safe unwrap
            }
            if self.matrix_a[i] == vec![Frac::new(T::zero(), T::one()); self.n + 1]
                && self.matrix_b[i] != Frac::new(T::zero(), T::one())
            {
                return Err(Unsolvable);
            }
        }
        Ok(pivots)
    }

    fn get_pivot(&self, row: usize) -> Option<usize> {
        for column in 0..self.m {
            if self.matrix_a[row][column] != Frac::new(T::zero(), T::one()) {
                return Some(column);
            }
        }
        None
    }

    fn get_leftmost_row(&self, row: usize) -> Option<usize> {
        let mut fake_zero = false;
        let mut leftmost = row;
        let mut min_left: usize = match self.get_pivot(row) {
            Some(s) => s,
            None => {
                fake_zero = true;
                0
            }
        };
        for i in row + 1..self.n {
            let current_pivot = match self.get_pivot(i) {
                Some(s) => s,
                None => continue,
            };
            if (current_pivot < min_left) | (fake_zero) {
                leftmost = i;
                min_left = current_pivot;
                fake_zero = false;
            }
        }
        if fake_zero {
            None
        } else {
            Some(leftmost)
        }
    }

    fn mul_row(&self, row: usize, multiplicator: Frac<T>) -> Result<Vec<Frac<T>>, ErrorCases> {
        let mut v: Vec<Frac<T>> = Vec::new();
        for column in 0..self.m {
            v.push((self.matrix_a[row][column] * multiplicator)?);
        }
        v.push((self.matrix_b[row] * multiplicator)?);
        Ok(v)
    }

    fn divide_row(&mut self, row: usize, divisor: Frac<T>) -> Result<bool, ErrorCases> {
        for column in 0..self.m {
            self.matrix_a[row][column] = (self.matrix_a[row][column] / divisor)?;
        }
        self.matrix_b[row] = (self.matrix_b[row] / divisor)?;
        Ok(true)
    }

    fn get_max_abs_row(&self, row: usize, column: usize) -> Result<usize, ErrorCases> {
        let mut maxi = row;
        for k in row + 1..self.n {
            if self.matrix_a[k][column].abs()? > self.matrix_a[maxi][column].abs()? {
                maxi = k;
            }
        }
        Ok(maxi)
    }
}
