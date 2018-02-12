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

extern crate lib_xch;

mod testers;

use testers::{tester, tester_error};
use lib_xch::handler::ErrorCases::*;

#[test]
fn simples() {
    tester("H2O=H2+O2", 10, &[2, 2, 1]);
}

#[test]
fn high_performance() {
    tester("As2O3+Zn+HCl=AsH3+ZnCl2+H2O", 50, &[1, 6, 12, 2, 6, 3]);
}

#[test]
fn brackets() {
    tester("O2(O3(O)4O5(O))=O", 50, &[1, 15]);
}

#[test]
fn illegal_equation() {
    tester_error("AAAA", 10, &IllegalEquation);
    tester_error("AAAA==", 10, &IllegalEquation);
    tester_error("/A=A*", 10, &IllegalEquation);
    // The third situation can't impl.
}

#[test]
fn i32_overflow() {
    tester_error("(((A32767)32767)32434)54342=A", 30, &I32Overflow);
}

#[test]
fn match_error() {
    tester_error("(((A))))=B", 10, &MatchError);
}

#[test]
fn split_error() {
    tester_error("+=A", 10, &SplitError);
}

#[test]
fn no_tokens() {
    // No example yet.
}

#[test]
fn not_found() {
    // No example yet.
}

#[test]
fn no_answer() {
    tester_error("A=B", 32, &NoAnswer);
}

#[test]
fn i32_parse_error() {
    tester_error("(A)111111111111111111111111111=A", 10, &I32ParseError);
}
