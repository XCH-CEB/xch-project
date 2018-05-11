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

use lib_xch::api::handler::ErrorCases::*;
use testers::{tester, tester_error};

#[test]
fn simples() {
    tester::<i32>("H2O=H2+O2", &[2, 2, 1]);
    tester::<i32>("Al+Fe3O4=Fe+Al2O3", &[8, 3, 9, 4]);
    tester::<i32>("FeS2+O2=Fe2O3+SO2", &[4, 11, 2, 8]);
    tester::<i32>("As2O3+Zn+HCl=AsH3+ZnCl2+H2O", &[1, 6, 12, 2, 6, 3]);
    tester::<i32>("ABCDE=ABCDE", &[1, 1]);
    tester::<i32>(
        "K4Fe(CN)6+H2SO4+H2O=K2SO4+FeSO4+(NH4)2SO4+CO",
        &[1, 6, 6, 2, 1, 3, 6],
    );
    tester::<i32>("Al2(SO4)3+NaOH=Na2SO4+Al(OH)3", &[1, 6, 3, 2]);
    tester::<i32>("CuSO4+NaOH=Na2SO4+Cu(OH)2", &[1, 2, 1, 1]);
    tester::<i32>("Fe(OH)3+H2SO4=Fe2(SO4)3+H2O", &[2, 3, 1, 6]);
}

#[test]
fn high_performance() {
    tester::<i32>(
        "H2+Ca(CN)2+NaAlF4+FeSO4+MgSiO3+KI+H3PO4+PbCrO4+BrCl+CF2Cl2+SO2=PbBr2+CrCl3+MgCO3+KAl(OH)4+Fe(SCN)3+PI3+Na2SiO3+CaF2+H2O",
        &[88, 15, 6, 10, 3, 6, 2, 6, 12, 3, 20, 6, 6, 3, 6, 10, 2, 3, 15, 79],
    );
}

#[test]
fn brackets() {
    tester::<i32>("O2(O3(O)4O5(O))=O", &[1, 15]);
}

#[test]
fn illegal_equation() {
    tester_error::<i32>("AAAA", &IllegalEquation);
    tester_error::<i32>("AAAA==", &IllegalEquation);
    tester_error::<i32>("/A=A*", &IllegalEquation);
    // The third situation can't impl.
}

#[test]
fn overflow() {
    tester_error::<i32>("(((A32767)32767)32434)54342=A", &Overflow);
}

#[test]
fn match_error() {
    tester_error::<i32>("(((A))))=B", &MatchError);
}

#[test]
fn split_error() {
    tester_error::<i32>("+=A", &SplitError);
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
fn abs_error() {
    // No example yet.
}

#[test]
fn unsolvable() {
    // No example yet.
}

#[test]
fn no_answer() {
    tester_error::<i32>("A=B", &NoAnswer);
    tester_error::<i32>("A+A=B", &NoAnswer); // issue #1
    tester_error::<i32>("A+A=A+B", &NoAnswer); // issue #1
    tester_error::<i32>("A+A=AA+B", &NoAnswer); // issue #1
    tester_error::<i32>("KClO3+HCl=KCl+ClO2+Cl2+H2O", &NoAnswer); // INP Model can't solve it.
}

#[test]
fn parse_error() {
    tester_error::<i32>("(A)111111111111111111111111111=A", &ParseError);
}
