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
use crate::api::traits::CheckedType;

pub struct ASTTree<T: CheckedType> {
    tree: Tree<ASTNode<T>>,
    nodes: Vec<NodeId>,
    index: usize,
}

impl<T: CheckedType> ASTTree<T> {
    pub fn new() -> Self {
        let mut tree: Tree<ASTNode<T>> = TreeBuilder::new().build();
        let mut nodes: Vec<NodeId> = Vec::new();
        nodes.push(
            tree.insert(Node::new(ASTNode::new(NodeType::MoleculeGroup)), AsRoot)
                .unwrap(), // By using `AsRoot`, it always return `Ok(_)`, so it's safe.
        );
        Self {
            tree,
            nodes,
            index: 0,
        }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn change_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn new_node(&mut self, nodetype: NodeType<T>) -> Result<usize, NodeIdError> {
        self.nodes.push(self.tree.insert(
            Node::new(ASTNode::new(nodetype)),
            UnderNode(&self.nodes[self.index]),
        )?);
        Ok(self.nodes.len() - 1)
    }

    pub fn to_atomdict(&self) -> Result<AtomDict<T>, NodeIdError> {
        self.tree
            .get(&self.tree.root_node_id().unwrap())?
            .data()
            .to_atomdict(&self.tree.root_node_id().unwrap(), &self.tree)
    }
}
