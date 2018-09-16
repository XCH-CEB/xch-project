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

use id_tree::{
    InsertBehavior::{AsRoot, UnderNode},
    Node, NodeId, NodeIdError, Tree, TreeBuilder,
};
// inside uses
use super::{super::atomdict::AtomDict, node::ASTNode, node::NodeType};
use api::{handler::ErrorCases, traits::CheckedType};

pub struct ASTTree<T: CheckedType> {
    tree: Tree<ASTNode<T>>,
    root_id: NodeId,
}

impl<T: CheckedType> ASTTree<T> {
    pub fn new() -> Result<Self, ErrorCases> {
        let mut tree: Tree<ASTNode<T>> = TreeBuilder::new().build();
        let root_id = match tree.insert(Node::new(ASTNode::new(NodeType::MoleculeGroup)), AsRoot) {
            Ok(s) => s,
            Err(_) => return Err(ErrorCases::ParserError("[AST] NodeId Error".to_string())),
        };
        Ok(Self { tree, root_id })
    }

    pub fn new_node(
        &mut self,
        nodetype: NodeType<T>,
        parent: &NodeId,
    ) -> Result<NodeId, NodeIdError> {
        self.tree
            .insert(Node::new(ASTNode::new(nodetype)), UnderNode(parent))
    }

    pub fn get_root_id(&self) -> NodeId {
        (*self.tree.root_node_id().unwrap()).clone() // Root Node must be exist
    }

    pub fn to_atomdict(&self) -> Result<AtomDict<T>, NodeIdError> {
        self.tree
            .get(&self.root_id)?
            .data()
            .to_atomdict(&self.root_id, &self.tree)
    }
}
