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

use crate::public::failures::ErrorCases;
use id_tree::{NodeId, Tree};
// inside use(s)
use self::NodeType::{Atom, Molecule, MoleculeGroup, ParenthesisWrapper};
use super::{super::atomdict::AtomDict, treebuilder::F};
use crate::public::traits::CheckedType;

pub enum NodeType<T: CheckedType> {
    Atom(String, T),       // `Symbol` and `Suffix_Operand`
    ParenthesisWrapper(T), // `Suffix_Operand`
    Molecule(T, T),        // `Prefix_Operand` and `Electronic_Charge`
    MoleculeGroup,         // No Attributes contained
}

pub struct ASTNode<T: CheckedType> {
    nodetype: NodeType<T>,
}

impl<T: CheckedType> ASTNode<T> {
    pub fn new(nodetype: NodeType<T>) -> Self {
        Self { nodetype }
    }

    pub fn to_atomdict(
        &self,
        node_id: &NodeId,
        tree: &Tree<ASTNode<T>>,
    ) -> Result<AtomDict<T>, ErrorCases> {
        match &self.nodetype {
            Atom(s, o) => {
                let mut a = AtomDict::<T>::new();
                a.insert(s.to_string(), *o);
                Ok(a)
            }
            Molecule(o, c) => {
                let mut a = AtomDict::<T>::new();
                a.insert("e".to_string(), *c);
                Ok(tree.children_ids(node_id).map_err(F)?.fold(a, |b, c| {
                    b + tree.get(c).unwrap().data().to_atomdict(c, tree).unwrap()
                }) * *o)
            }
            ParenthesisWrapper(o) => {
                let a = AtomDict::<T>::new();
                Ok(tree.children_ids(node_id).map_err(F)?.fold(a, |b, c| {
                    b + tree.get(c).unwrap().data().to_atomdict(c, tree).unwrap()
                }) * *o)
            }
            MoleculeGroup => {
                let a = AtomDict::<T>::new();
                Ok(tree.children_ids(node_id).map_err(F)?.fold(a, |b, c| {
                    b + tree.get(c).unwrap().data().to_atomdict(c, tree).unwrap()
                }))
            }
        }
    }
}
