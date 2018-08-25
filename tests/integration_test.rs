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
fn solve() {
    tester::<i32>("H2O=H2+O2", &[&[2, 2, 1]]);
    tester::<i32>("Al+Fe3O4=Fe+Al2O3", &[&[8, 3, 9, 4]]);
    tester::<i32>("FeS2+O2=Fe2O3+SO2", &[&[4, 11, 2, 8]]);
    tester::<i32>("As2O3+Zn+HCl=AsH3+ZnCl2+H2O", &[&[1, 6, 12, 2, 6, 3]]);
    tester::<i32>("ABCDE=ABCDE", &[&[1, 1]]);
    tester::<i32>(
        "K4Fe(CN)6+H2SO4+H2O=K2SO4+FeSO4+(NH4)2SO4+CO",
        &[&[1, 6, 6, 2, 1, 3, 6]],
    );
    tester::<i32>("Al2(SO4)3+NaOH=Na2SO4+Al(OH)3", &[&[1, 6, 3, 2]]);
    tester::<i32>("CuSO4+NaOH=Na2SO4+Cu(OH)2", &[&[1, 2, 1, 1]]);
    tester::<i32>("Fe(OH)3+H2SO4=Fe2(SO4)3+H2O", &[&[2, 3, 1, 6]]);
    tester::<i32>(
        "KClO3+HCl=KCl+ClO2+Cl2+H2O",
        &[&[-4, 0, -4, -6, 3, 0], &[5, 6, 5, 6, 0, 3]],
    );
    tester::<i32>(
        "Cu+HNO3=Cu(NO3)2+NO+NO2+H2O",
        &[&[-1, 0, -1, -2, 4, 0], &[3, 8, 3, 2, 0, 4]],
    );
    tester::<i32>(
        "HOC6H2(NO2)2SO3H+NH4OH=HOC6H2(NO2)2SO3NH4+H2O",
        &[&[1, 1, 1, 1]],
    );
    tester::<i32>(
        "K2Cr2O7+CH3CH2OH+H2SO4=Cr2(SO4)3+CH3COOH+K2SO4+H2O",
        &[&[2, 3, 8, 2, 3, 2, 11]],
    );
    tester::<i32>(
        "CoSO4+BaCO3+HCN=Ba3(Co(CN)6)2+BaSO4+H2SO4+CO2+H2+H2O",
        &[
            &[2, 0, 12, 1, -3, 5, 0, 1, 0],
            &[0, 1, 0, 0, 1, -1, 1, 0, 1],
        ],
    );
    tester::<i32>(
        "Co(NO3)2+KNO2+CH3COOH=CH3COOK+K3(Co(NO2)6)+KNO3+NO+H2O",
        &[&[-1, -4, 0, 0, -1, -1, 1, 0], &[2, 11, 2, 2, 2, 3, 0, 1]],
    );
    tester::<i32>(
        "Co(NO3)2+(NH4)2CO2+NH3+O2=NH4NO3+Co(NH3)4CO3NO3+H2O",
        &[&[4, 4, 8, -1, 2, 4, 0], &[0, 0, 2, 2, 1, 0, 1]],
    );
    tester_error::<i32>("A=B", &ZeroSolution);
    tester::<i32>("A+A=B", &[&[-1, 1, 0]]);
    tester::<i32>("A+A=A+B", &[&[-1, 1, 0, 0], &[1, 0, 1, 0]]);
    tester::<i32>("A+A=AA+B", &[&[-1, 1, 0, 0], &[2, 0, 1, 0]]);
    tester::<i32>(
        "H2+Ca(CN)2+NaAlF4+FeSO4+MgSiO3+KI+H3PO4+PbCrO4+BrCl+CF2Cl2+SO2=PbBr2+CrCl3+MgCO3+KAl(OH)4+Fe(SCN)3+PI3+Na2SiO3+CaF2+H2O",
        &[&[88, 15, 6, 10, 3, 6, 2, 6, 12, 3, 20, 6, 6, 3, 6, 10, 2, 3, 15, 79]],
    );
    tester::<i32>("O2(O3(O)4O5(O))=O", &[&[1, 15]]);
    tester::<i32>("CuSO4.5H2O=CuSO4+H2O", &[&[1, 1, 5]]);
    tester::<i32>(
        "NO+MnO4<e->+H<e+>=Mn<2e+>+NO3<e->+H2O",
        &[&[5, 3, 4, 3, 5, 2]],
    );
    tester::<i32>(
        "Pb(N3)2+Cr(MnO4)2=Cr2O3+MnO2+Pb3O4+NO",
        &[&[15, 44, 22, 88, 5, 90]],
    );
    tester::<i32>(
        "NH4<e+>+I<e->+ClO<e->=NHI2+NH3+Cl<e->+H2O",
        &[&[2, 2, 2, 1, 1, 2, 2]],
    );
    tester::<i32>(
        "HXeO4<e->+OH<e->=XeO6<4e->+Xe+O2+H2O",
        &[&[2, -2, 0, 2, 3, 0], &[4, 8, 3, 1, 0, 6]],
    );
    tester::<i32>(
        "CuS+CN<e->+OH<e->=Cu(CN)4<3e->+NCO<e->+S+S<2e->+H2O",
        &[&[2, 8, 0, 2, 0, 1, 1, 0], &[-2, -7, 2, -2, 1, -2, 0, 1]],
    );
    tester::<i32>(
        "NH4ClO4+HNO3+HCl=HClO4+N2O+Cl2+H2O",
        &[
            &[-1, 3, 2, 1, 1, 0, 0],
            &[-4, 4, 11, -1, 0, 4, 0],
            &[1, -1, -1, 0, 0, 0, 1],
        ],
    );
    tester::<i32>(
        "(NH4)2CO3+H2O=NH4OH+NH3+CO2+H2O",
        &[
            &[0, -1, -1, 1, 0, 0],
            &[1, 1, 2, 0, 1, 0],
            &[0, 1, 0, 0, 0, 1],
        ],
    );
    tester::<i32>(
        "Co(NO3)2+KNO2+CH3COOH=CH3COOK+K3(Co(NO2)6)+KNO3+NO+H2O",
        &[&[-1, -4, 0, 0, -1, -1, 1, 0], &[2, 11, 2, 2, 2, 3, 0, 1]],
    );
}

#[test]
fn error() {
    tester_error::<i32>("AAA", &ParserError(" --> 1:4\n  |\n1 | AAA\n  |    ^---\n  |\n  = expected num, atom_name, electron, or parenthesis_wrapper".to_string()));
    tester_error::<i32>(
        "AAAA==",
        &ParserError(" --> 1:6\n  |\n1 | AAAA==\n  |      ^---\n  |\n  = expected exp".to_string()),
    );
    tester_error::<i32>(
        "/A=A*",
        &ParserError(" --> 1:1\n  |\n1 | /A=A*\n  | ^---\n  |\n  = expected molecule".to_string()),
    );
    tester_error::<i32>("A=B", &ZeroSolution);
}
