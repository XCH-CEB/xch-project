// Copyright 2019 LEXUGE
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

use criterion::{criterion_group, criterion_main, Criterion};
use lib_xch::public::{
    handler::Handler,
    traits::{CheckedCalc, CheckedType},
};

const EQU_1: &'static str = "H2+Ca(CN)2+NaAlF4+FeSO4+MgSiO3+KI+H3PO4+PbCrO4+BrCl+CF2Cl2+SO2=PbBr2+CrCl3+MgCO3+KAl(OH)4+Fe(SCN)3+PI3+Na2SiO3+CaF2+H2O";
const EQU_2: &'static str = "NH4ClO4+HNO3+HCl=HClO4+N2O+Cl2+H2O";

fn handle<T: CheckedType + CheckedCalc>(equ: &str)
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    Handler::<T>::new(equ).handle().unwrap();
}

fn parse<T: CheckedType + CheckedCalc>(equ: &str)
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    Handler::<T>::new(equ).parse().unwrap();
}

fn bench_handle(c: &mut Criterion) {
    c.bench_function("handle", |b| {
        b.iter(|| {
            handle::<isize>(EQU_1);
            handle::<isize>(EQU_2);
        })
    });
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse", |b| {
        b.iter(|| {
            parse::<isize>(EQU_1);
            parse::<i32>(EQU_2)
        })
    });
}

criterion_group!(benches, bench_parse, bench_handle);
criterion_main!(benches);
