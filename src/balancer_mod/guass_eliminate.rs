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
use handler::ErrorCases::{FreeVariables, Unsolvable};
use super::frac_util::Frac;

pub struct GuassianElimination {
    matrix_a: Vec<Vec<Frac>>, // A n*n matrix.
    matrix_b: Vec<Frac>,      // A n*1 matrix.
    n: usize,
}

impl GuassianElimination {
    pub fn new(matrix_a: Vec<Vec<Frac>>, matrix_b: Vec<Frac>, n: usize) -> Self {
        // Create a GuassianElimination Solution.
        Self {
            matrix_a: matrix_a,
            matrix_b: matrix_b,
            n: n,
        }
    }

    pub fn solve(&mut self) -> Result<Vec<Frac>, ErrorCases> {
        for i in 1..self.n + 1 {
            let mut j = i;
            let maxi = self.get_max_abs_row(i, j)?;
            if self.matrix_a[maxi][j].numerator != 0 {
                self.matrix_a.swap(i, maxi);
                self.matrix_b.swap(i, maxi); // swap row i and maxi in matrix_a and matrix_b
                {
                    let tmp = self.matrix_a[i][j];
                    self.divide_row(i, tmp)?;
                }
                for u in i + 1..self.n + 1 {
                    let mut v = self.mul_row(i, self.matrix_a[u][j])?; // v has n+2 elements
                    for (k, item) in v.iter().enumerate().take(self.n + 1).skip(1) {
                        self.matrix_a[u][k] = self.matrix_a[u][k].sub(*item)?;
                    }
                    self.matrix_b[u] = self.matrix_b[u].sub(v[self.n + 1])?;
                }
            }
        }

        let mut ans: Vec<Frac> = Vec::new();
        ans.push(self.matrix_b[self.n].div(self.matrix_a[self.n][self.n])?);
        for i in (1..self.n).rev() {
            let mut sum = Frac::new(0, 1);
            for j in i + 1..self.n + 1 {
                sum = sum.add(self.matrix_a[i][j].mul(ans[self.n - j])?)?;
            }
            ans.push(self.matrix_b[i].sub(sum)?.div(self.matrix_a[i][i])?);
        }
        self.check()?;
        Ok(ans) // x_{n} to x_{1}
    }

    fn check(&self) -> Result<bool, ErrorCases> {
        for i in 1..self.n + 1 {
            if self.matrix_a[i] == vec![Frac::new(0, 1); self.n + 1] {
                if self.matrix_b[i] == Frac::new(0, 1) {
                    return Err(FreeVariables);
                } else {
                    return Err(Unsolvable);
                }
            }
        }
        Ok(true)
    }

    fn mul_row(&self, row: usize, multiplicator: Frac) -> Result<Vec<Frac>, ErrorCases> {
        let mut v: Vec<Frac> = Vec::new();
        v.push(Frac::new(0, 1));
        for column in 1..self.n + 1 {
            v.push(self.matrix_a[row][column].mul(multiplicator)?);
        }
        v.push(self.matrix_b[row].mul(multiplicator)?);
        Ok(v)
    }

    fn divide_row(&mut self, row: usize, divisor: Frac) -> Result<bool, ErrorCases> {
        for column in 1..self.n + 1 {
            self.matrix_a[row][column] = self.matrix_a[row][column].div(divisor)?;
        }
        self.matrix_b[row] = self.matrix_b[row].div(divisor)?;
        Ok(true)
    }

    fn get_max_abs_row(&self, row: usize, column: usize) -> Result<usize, ErrorCases> {
        let mut maxi = row;
        for k in row + 1..self.n + 1 {
            if self.matrix_a[k][column].abs()? > self.matrix_a[maxi][column].abs()? {
                maxi = k;
            }
        }
        Ok(maxi)
    }
}
