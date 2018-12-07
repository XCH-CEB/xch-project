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

// Overall: This is the source code of the Delta-3 Parser.

use id_tree::NodeId;
use pest::{iterators::Pair, Parser};
use std::str::FromStr;
// inside uses
use super::{node::NodeType, tree::ASTTree};
use crate::api::{handler::ErrorCases, traits::CheckedType};

#[derive(Parser)]
#[grammar = "ast.pest"]
struct MoleculeParser;

pub struct ASTTreeBuilder;

impl ASTTreeBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse<T: CheckedType>(&self, formula: &str) -> Result<ASTTree<T>, ErrorCases> {
        let (mut tree, root_id) = ASTTree::<T>::new()?;
        let pairs = match MoleculeParser::parse(Rule::molecule_group, formula) {
            Ok(s) => s,
            Err(e) => return Err(ErrorCases::ParserError(e.to_string())),
        };
        for p in pairs {
            // The `pairs` only contains one Pair actually.
            self.build_tree(p, &mut tree, &root_id)?
        }
        Ok(tree)
    }

    fn build_tree<T: CheckedType>(
        &self,
        pair: Pair<Rule>,
        tree: &mut ASTTree<T>,
        parent: &NodeId,
    ) -> Result<(), ErrorCases> {
        match pair.as_rule() {
            Rule::atom => {
                let pairs = pair.into_inner().collect::<Vec<_>>();
                let atom_name = pairs[0].as_str().to_string();
                let operand = if pairs[pairs.len() - 1].as_rule() == Rule::num {
                    self.parse_from(&pairs[pairs.len() - 1].as_str())?
                } else {
                    T::one()
                };
                self.new_node_alias(tree, NodeType::Atom(atom_name, operand), parent)?;
                Ok(())
            }
            Rule::molecule => {
                let mut pairs = pair.into_inner().collect::<Vec<_>>();
                let prefix = if pairs[0].as_rule() == Rule::num {
                    self.parse_from(pairs[0].as_str())?
                } else {
                    T::one()
                };
                let charge = if pairs[pairs.len() - 1].as_rule() == Rule::electron {
                    let inner_pairs = pairs.pop().unwrap().into_inner().collect::<Vec<_>>();
                    let operand = if inner_pairs[0].as_rule() == Rule::num {
                        self.parse_from(inner_pairs[0].as_str())?
                    } else {
                        T::one()
                    };
                    self.parse_from(&format!(
                        "{}{}",
                        inner_pairs[inner_pairs.len() - 1].as_str(),
                        operand,
                    ))?
                } else {
                    T::zero()
                };
                let nodeid =
                    &self.new_node_alias(tree, NodeType::Molecule(prefix, charge), parent)?;
                for p in pairs {
                    self.build_tree(p, tree, nodeid)?;
                }
                Ok(())
            }
            Rule::parenthesis_wrapper => {
                let pairs = pair.into_inner().collect::<Vec<_>>();
                let suffix = if pairs[pairs.len() - 1].as_rule() == Rule::num {
                    self.parse_from(pairs[pairs.len() - 1].as_str())?
                } else {
                    T::one()
                };
                let nodeid =
                    &self.new_node_alias(tree, NodeType::ParenthesisWrapper(suffix), parent)?;
                for p in pairs {
                    self.build_tree(p, tree, nodeid)?;
                }
                Ok(())
            }
            Rule::molecule_group => {
                let pairs = pair.into_inner().collect::<Vec<_>>();
                for p in pairs {
                    self.build_tree(p, tree, parent)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn parse_from<T: FromStr>(&self, s: &str) -> Result<T, ErrorCases> {
        match s.parse::<T>() {
            Ok(s) => Ok(s),
            Err(_) => Err(ErrorCases::ParserError(format!(
                "{} '{}'",
                "Can't parse", s
            ))),
        }
    }

    fn new_node_alias<T: CheckedType>(
        &self,
        tree: &mut ASTTree<T>,
        nodetype: NodeType<T>,
        parent: &NodeId,
    ) -> Result<NodeId, ErrorCases> {
        match tree.new_node(nodetype, &parent) {
            Ok(s) => Ok(s),
            Err(_) => Err(ErrorCases::ParserError("[AST] NodeId Error".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ASTTreeBuilder;
    use std::collections::HashMap;

    #[test]
    fn parse() {
        let builder = ASTTreeBuilder::new();
        assert_eq!(
            builder
                .parse::<i32>("((NH4)3(PO4.12MoO3<12e->).2H2O)<32e+>") // This chemical formula is unreal
                .unwrap()
                .to_atomdict()
                .unwrap()
                .get_dict(),
            &[
                ("H".to_string(), 16),
                ("e".to_string(), -112),
                ("N".to_string(), 3),
                ("O".to_string(), 42),
                ("P".to_string(), 1),
                ("Mo".to_string(), 12)
            ]
            .iter()
            .cloned()
            .collect::<HashMap<String, i32>>()
        );
    }
}
