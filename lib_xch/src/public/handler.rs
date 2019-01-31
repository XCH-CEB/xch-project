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

//! The major part of APIs.
//! The API which balances the Chemical Equation by equation.
//!
//!  It provides a set of Basic Solutions. You can get infinite number of solutions by doing linear combination on the Basic Solution Set.
//!
//!  You can use any type which implemented the trait `api::traits::CheckedType`
//!

use std::collections::HashMap;
// inside use(s)
use super::{
    failures::ErrorCases,
    traits::{CheckedCalc, CheckedType},
    types::DataSet,
};
use crate::{
    balancer::handler::balancer,
    parser::handler::parser,
    public::{cell::Cell, structs::ChemicalEquation},
};

/// A handler which store the equation and other information
pub struct Handler<'a, T> {
    equ: &'a str,
    ds: HashMap<&'static str, Vec<Vec<Cell<T>>>>,
    cd: ChemicalEquation,
}

impl<'a, T: CheckedType + CheckedCalc> Handler<'a, T>
where
    std::num::ParseIntError: std::convert::From<<T as num::Num>::FromStrRadixErr>
        + std::convert::From<<T as std::str::FromStr>::Err>,
{
    /// Create a `Handler` by given equation
    pub fn new(equ: &'a str) -> Self {
        Handler {
            equ,
            ds: HashMap::new(),
            cd: ChemicalEquation::new(),
        }
    }
    /// Parse and balance the equation. If it has been parsed, then just balance it.
    ///
    /// If the equation can balance, function would return `Ok((ChemicalEquation, Vec<Vec<T>>))` which contains the answer.
    ///
    /// If not, it would return `Err(ErrorCases)` which contains the failure.
    ///
    /// # Panics
    ///
    /// -  A large number (bigger than [`usize::MAX`](https://doc.rust-lang.org/std/usize/constant.MAX.html)) of formula may cause **panic**. Because it is using `Vec`.
    ///
    /// And in the other failed situation, it'll return  `ErrorCases`.
    pub fn handle(&mut self) -> Result<DataSet<&T>, ErrorCases> {
        self.parse()?;
        self.balance()?;
        Ok((&self.cd, fromcell(&self.ds["Balancer"])?))
    }

    /// Parse the equation
    pub fn parse(&mut self) -> Result<DataSet<&T>, ErrorCases> {
        match parser::<Cell<T>>(self.equ)? {
            (cd, data) => {
                self.cd = cd;
                self.ds.insert("Parser", data);
            }
        }
        Ok((&self.cd, fromcell(&self.ds["Parser"])?))
    }

    // Balance the equation
    fn balance(&mut self) -> Result<(), ErrorCases> {
        self.ds.insert(
            "Balancer",
            balancer::<Cell<T>>((&self.cd, &self.ds["Parser"]))?,
        );
        Ok(())
    }
}
// All `false` => `true` (It didn't overflow)
fn check_tag<T>(v: &[Vec<Cell<T>>]) -> bool {
    v.iter().all(|x| x.iter().all(|x| !x.get_tag()))
}

fn fromcell<T>(v: &[Vec<Cell<T>>]) -> Result<Vec<Vec<&T>>, ErrorCases> {
    if !check_tag(v) {
        return Err(ErrorCases::Overflow);
    }
    Ok(v.iter()
        .map(|x| x.iter().map(|c| c.get_data()).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
