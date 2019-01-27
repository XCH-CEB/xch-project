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

// Overall: This is the source code of the Delta-3 Parser.

use pest::Parser;
use pest_derive::Parser;
// inside use(s)
use super::{ast::treebuilder::ASTTreeBuilder, datastructure::TableDesc};
use crate::public::{
    calc::{safe_calc, Operator},
    failures::ErrorCases,
    structs::ChemicalEquation,
    traits::CheckedType,
};

#[derive(Parser)]
#[grammar = "ast.pest"]
struct EquParser;

pub fn parser<T: CheckedType>(equ: &str) -> Result<(ChemicalEquation, Vec<Vec<T>>), ErrorCases> {
    let builder = ASTTreeBuilder::new();
    let exps = EquParser::parse(Rule::equ, equ)
        .map_err(|e| ErrorCases::ParserError(e.to_string()))?
        .collect::<Vec<_>>();
    let mut ce_desc = ChemicalEquation {
        left: exps[0].clone().into_inner().count(),
        right: exps[1].clone().into_inner().count(),
        sum: 0,
    };
    ce_desc.sum = safe_calc(&ce_desc.left, &ce_desc.right, &Operator::Add)?;
    let mut table = TableDesc::new(ce_desc.sum);
    for (location, formula) in exps[0].clone().into_inner().enumerate() {
        let atomdict = builder.parse(formula.as_str())?.to_atomdict()?;
        table.store_in_table(&atomdict, location, false);
    }
    for (location, formula) in exps[1].clone().into_inner().enumerate() {
        let atomdict = builder.parse(formula.as_str())?.to_atomdict()?;
        table.store_in_table(&atomdict, location + ce_desc.left, true);
    }
    Ok((ce_desc, table.get_list()))
}
