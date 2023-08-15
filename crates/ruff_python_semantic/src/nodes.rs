use std::ops::Index;

use ruff_index::{newtype_index, IndexVec};
use ruff_python_ast::node::AnyNodeRef;

/// Id uniquely identifying an AST node in a program.
///
/// Using a `u32` is sufficient because Ruff only supports parsing documents with a size of max
/// `u32::max` and it is impossible to have more nodes than characters in the file. We use a
/// `NonZeroU32` to take advantage of memory layout optimizations.
#[newtype_index]
#[derive(Ord, PartialOrd)]
pub struct NodeId;

/// An AST node in a program, along with a pointer to its parent node (if any).
#[derive(Debug)]
struct NodeWithParent<'a> {
    /// A pointer to the AST node.
    node: AnyNodeRef<'a>,
    /// The ID of the parent of this node, if any.
    parent: Option<NodeId>,
}

/// The nodes of a program indexed by [`NodeId`]
#[derive(Debug, Default)]
pub struct Nodes<'a> {
    nodes: IndexVec<NodeId, NodeWithParent<'a>>,
}

impl<'a> Nodes<'a> {
    /// Inserts a new AST node into the tree and returns its unique ID.
    pub(crate) fn insert(&mut self, node: AnyNodeRef<'a>, parent: Option<NodeId>) -> NodeId {
        self.nodes.push(NodeWithParent { node, parent })
    }

    /// Return the [`NodeId`] of the parent node.
    #[inline]
    pub fn parent_id(&self, node_id: NodeId) -> Option<NodeId> {
        self.nodes[node_id].parent
    }

    /// Returns an iterator over all [`NodeId`] ancestors, starting from the given [`NodeId`].
    pub(crate) fn ancestor_ids(&self, node_id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        std::iter::successors(Some(node_id), |&node_id| self.nodes[node_id].parent)
    }
}

impl<'a> Index<NodeId> for Nodes<'a> {
    type Output = AnyNodeRef<'a>;

    #[inline]
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index].node
    }
}
