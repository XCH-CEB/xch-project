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
use std::i32;
// inside uses
use structs::ChemicalEquation;

pub fn xch_try(
    f: i32,
    searching_range: i32,
    traversal: &mut Vec<i32>,
    list: &Vec<Vec<i32>>,
    chmcl_f_sut: &ChemicalEquation,
    len: usize,
) -> bool {
    if f == chmcl_f_sut.sum + 1 {
        if check(traversal, list, chmcl_f_sut, len) == true {
            return true;
        }
    } else {
        for i in 1..searching_range + 1 {
            traversal.push(i);
            if xch_try(f + 1, searching_range, traversal, list, chmcl_f_sut, len) == true {
                return true;
            }
            traversal.pop();
        }
    }
    false
}

fn check(
    traversal: &mut Vec<i32>,
    list: &Vec<Vec<i32>>,
    chmcl_f_sut: &ChemicalEquation,
    len: usize,
) -> bool {
    let mut tmp1: i32;
    let mut tmp2: i32;
    for i in 1..len + 1 {
        tmp1 = 0;
        tmp2 = 0;
        for j in 1..chmcl_f_sut.left_num as usize + 1 {
            let tmp: i32;
            tmp = list[i][j]
                .checked_mul(traversal[j - 1])
                .expect("[ERROR] i32 overflow");
            tmp1 = tmp1.checked_add(tmp).expect("[ERROR] i32 overflow");
        }
        for j in chmcl_f_sut.left_num as usize + 1..chmcl_f_sut.sum as usize + 1 {
            let tmp: i32;
            tmp = list[i][j]
                .checked_mul(traversal[j - 1])
                .expect("[ERROR] i32 overflow");
            tmp2 = tmp2.checked_add(tmp).expect("[ERROR] i32 overflow");
        }
        if tmp1 != tmp2 {
            return false;
        }
    }
    true
}
