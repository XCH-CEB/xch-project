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

use handler::handler_api;

fn tester(equ: &str, range: i32, v: Vec<i32>) {
    let foo = match handler_api(equ.to_string(), range) {
        Ok(v) => v,
        Err(_) => panic!("Failed!"),
    };
    assert_eq!(foo, v);
}

fn tester_error(equ: &str, range: i32, err: &str) {
    let foo = match handler_api(equ.to_string(), range) {
        Ok(_) => panic!("Failed!"),
        Err(s) => s.error_message,
    };
    assert_eq!(foo, err.to_string());
}
#[test]
fn test_simples() {
    tester("H2O=H2+O2", 10, vec![2, 2, 1]);
}

#[test]
fn test_high_performance() {
    tester("As2O3+Zn+HCl=AsH3+ZnCl2+H2O", 50, vec![1, 6, 12, 2, 6, 3]);
}

#[test]
fn test_brackets() {
    tester("O2(O3(O)4O5(O))=O", 50, vec![1, 15]);
}

#[test]
fn test_illegal_equation() {
    tester_error("A+()=B", 10, "[ERROR] No tokens!");
    tester_error("(((A))))=B", 10, "[ERROR] Can't match!");
}

#[test]
fn test_i32_overflow() {
    tester_error("(((A32767)32767)32434)54342=A", 30, "[ERROR] i32 overflow");
}

#[test]
fn test_no_answer() {
    tester_error("A=V", 32, "No answer");
}
