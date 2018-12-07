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

use crate::na::base::{Dynamic, MatrixMN};
use num::{rational::Ratio, One, Signed, Zero};
// inside uses
use crate::api::{
    handler::{ErrorCases, ErrorCases::ZeroSolution},
    traits::CheckedType,
};

pub struct GaussianElimination<T: CheckedType> {
    matrix_a: MatrixMN<Ratio<T>, Dynamic, Dynamic>, // A n*m matrix.
    n: usize,
    m: usize,
}

impl<T: CheckedType> GaussianElimination<T> {
    pub fn new(matrix_a: MatrixMN<Ratio<T>, Dynamic, Dynamic>) -> Self {
        // Create a GaussianElimination Solution.
        let (n, m) = matrix_a.shape();
        Self { matrix_a, n, m }
    }

    pub fn solve(mut self) -> Result<Vec<Vec<Ratio<T>>>, ErrorCases> {
        // The Gaussian-Jordan Algorithm
        let mut var_table = Vec::<usize>::new();
        for i in 0..self.n {
            let mostleft_row = match self.get_leftmost_row(i) {
                Some(s) => s,
                None => continue,
            };
            let j = match self.get_pivot(mostleft_row) {
                Some(s) => {
                    var_table.push(s);
                    s
                }
                None => continue, // if most left row has no pivot, just continue.
            };
            let max_row = self.get_max_abs_row(i, j);
            if self.matrix_a[(max_row, j)] != Ratio::<T>::zero() {
                self.matrix_a.swap_rows(i, max_row); // swap row i and maxi in matrix_a
                {
                    let tmp = &(self.matrix_a.row(i) / self.matrix_a[(i, j)]);
                    self.matrix_a.row_mut(i).copy_from(tmp);
                }
                for u in i + 1..self.n {
                    let v = self.matrix_a.row(i) * self.matrix_a[(u, j)];
                    for (k, item) in v.iter().enumerate().take(self.m) {
                        self.matrix_a[(u, k)] -= *item; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                    }
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
                let v = self.matrix_a.row(i) * self.matrix_a[(u, j)];
                for (k, item) in v.iter().enumerate().take(self.m) {
                    self.matrix_a[(u, k)] -= *item; // A_{u}=A_{u}-A_{u}{j}*A_{i}
                }
            }
        } // RREF
        let v = (0..self.n)
            .filter(|i| self.matrix_a.row(*i).iter().all(|e| e.is_zero()))
            .collect::<Vec<_>>();
        self = self.simplify(v); // eliminate the zero rows
        var_table = (0..self.m)
            .filter(|e| !var_table.contains(&e))
            .collect::<Vec<_>>(); // get free variables table
        var_table.iter().for_each(|x| {
            let tmp = self.matrix_a.column(*x) * -Ratio::<T>::one();
            self.matrix_a.column_mut(*x).copy_from(&tmp)
        });
        let mut ans = var_table
            .iter()
            .map(|i| self.matrix_a.column(*i).iter().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let len = var_table.len();
        for (i, j) in var_table.into_iter().enumerate() {
            for (v, item) in ans.iter_mut().enumerate().take(len) {
                if i == v {
                    item.insert(j, Ratio::<T>::one());
                } else {
                    item.insert(j, Ratio::<T>::zero());
                }
            }
        }
        if ans.is_empty() {
            Err(ZeroSolution)
        } else {
            Ok(ans)
        }
    }

    fn simplify(mut self, list: Vec<usize>) -> Self {
        for i in list.into_iter().rev() {
            self.matrix_a = self.matrix_a.remove_row(i);
        }
        let (n, m) = self.matrix_a.shape();
        Self {
            matrix_a: self.matrix_a,
            n,
            m,
        }
    }

    fn get_pivot(&self, row: usize) -> Option<usize> {
        for column in 0..self.m {
            if self.matrix_a[(row, column)] != Ratio::<T>::zero() {
                return Some(column);
            }
        }
        None
    }

    fn get_leftmost_row(&self, row: usize) -> Option<usize> {
        let mut lock = false;
        // Use `lock` to prevent calculation from `usize` Overflow
        let mut mostleft_row = row;
        let mut min_left: usize = match self.get_pivot(row) {
            Some(s) => s,
            None => {
                lock = true;
                0
            }
        };
        for i in row + 1..self.n {
            let current_pivot = match self.get_pivot(i) {
                Some(s) => s,
                None => continue,
            };
            if (current_pivot < min_left) | (lock) {
                mostleft_row = i;
                min_left = current_pivot;
                lock = false;
            }
        }
        if lock {
            None
        } else {
            Some(mostleft_row)
        }
    }

    fn get_max_abs_row(&self, row: usize, column: usize) -> usize {
        let mut maxi = row;
        for k in row + 1..self.n {
            if self.matrix_a[(k, column)].abs() > self.matrix_a[(maxi, column)].abs() {
                maxi = k;
            }
        }
        maxi
    }
}
