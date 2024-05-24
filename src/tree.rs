use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, ser::SerializeStruct, Serialize, Serializer};

use crate::error::Error::{InvalidOperation, RootNodeAlreadyPresent};
use crate::prelude::Node;

/// The strategy to use when removing a node from the tree.
///
/// This enum represents the strategy to use when removing a node from the tree. The `RetainChildren`
/// strategy retains the children of the node when the node is removed. The `RemoveNodeAndChildren`
/// strategy removes the node and its children when the node is removed.
#[derive(Clone, Debug, Copy)]
pub enum NodeRemovalStrategy {
	/// Retain the children of the node. This means that the children of the node are attached to the
	/// parent of the node when the node is removed. So the children of the node become children of the
	/// parent of the node.
	RetainChildren,
	/// Remove the node and all subsequent children. This means that the node and its children are
	/// removed from the tree when the node is removed. All the subsequent grand children of the node are
	/// removed from the tree.
	RemoveNodeAndChildren,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, Copy)]
pub enum TraversalStrategy {
	PreOrder,
	PostOrder,
	InOrder,
}

pub type SubTree<Q, T> = Tree<Q, T>;

/// A tree data structure.
///
/// This struct represents a tree data structure. A tree is a data structure that consists of nodes
/// connected by edges. Each node has a parent node and zero or more child nodes. The tree has a root
/// node that is the topmost node in the tree. The tree can be used to represent hierarchical data
/// structures such as file systems, organization charts, and family trees.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Tree;
///
/// let tree: Tree<i32, i32> = Tree::new();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone,
		T: PartialEq + Eq + Clone,
{
	nodes: Vec<Node<Q, T>>,
}

impl<Q, T> Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone + Display + Hash,
		T: PartialEq + Eq + Clone,
{
	/// Create a new tree.
	///
	/// This method creates a new tree with no nodes.
	///
	/// # Returns
	///
	/// A new tree with no nodes.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Tree;
	///
	/// let tree: Tree<i32, i32> = Tree::new();
	/// ```
	pub fn new() -> Self {
		Tree::default()
	}

	/// Add a node to the tree.
	///
	/// This method adds a node to the tree. The node is added as a child of the parent node with the
	/// given parent id. If the parent id is `None`, the node is added as a root node. The node id is
	/// used to identify the node and the value is the value of the node. The value can be used to store
	/// any data that you want to associate with the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	/// * `value` - The value of the node.
	/// * `parent_id` - The id of the parent node. If `None`, the node is added as a root node.
	///
	/// # Returns
	///
	/// The id of the node that was added to the tree. However, if no parent id is provided and the tree already
	/// has a root node, an error is returned.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Tree, Node};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// ```
	pub fn add_node(
		&mut self,
		node: Node<Q, T>,
		parent_id: Option<&Q>,
	) -> crate::prelude::Result<Q> {
		if let Some(parent_id) = parent_id {
			if let Some(parent) = self.nodes.iter().find(|n| &n.get_node_id() == parent_id) {
				parent.add_child(node.clone());
			}
		} else if self.get_root_node().is_some() {
			return Err(RootNodeAlreadyPresent);
		}
		self.nodes.push(node.clone());
		Ok(node.get_node_id())
	}

	/// Get a node in the tree.
	///
	/// This method gets the node with the given node id in the tree.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	///
	/// # Returns
	///
	/// The node with the given node id in the tree or `None` if the node is not found.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// let node_id = tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_node(&node_id), Some(node));
	/// ```
	pub fn get_node(&self, node_id: &Q) -> Option<Node<Q, T>> {
		self.nodes
			.iter()
			.find(|n| &n.get_node_id() == node_id)
			.cloned()
	}

	/// Get the root node of the tree.
	///
	/// This method gets the root node of the tree. The root node is the topmost node in the tree. The
	/// root node has no parent node.
	///
	/// # Returns
	///
	/// The root node of the tree or `None` if the tree has no root node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_root_node(), Some(node));
	/// ```
	pub fn get_root_node(&self) -> Option<Node<Q, T>> {
		self.nodes
			.iter()
			.find(|n| n.get_parent().is_none())
			.cloned()
	}

	/// Get the height of the tree.
	///
	/// This method gets the height of the tree. The height of the tree is the length of the longest path
	/// from the root node to a leaf node. The height of the tree is the number of edges on the longest
	/// path from the root node to a leaf node.
	///
	/// # Returns
	///
	/// The height of the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// assert_eq!(tree.get_node_height(&node_1), 2);
	/// ```
	pub fn get_node_height(&self, node_id: &Q) -> i32 {
		let node = self.get_node(node_id).unwrap();
		let children = node.get_children();
		if children.is_empty() {
			return 0;
		}
		let mut height = 0;
		for child in children {
			let child_height = self.get_node_height(&child);
			if child_height > height {
				height = child_height;
			}
		}
		height + 1
	}

	/// Get the depth of a node in the tree.
	///
	/// This method gets the depth of a node in the tree. The depth of a node is the length of the path
	/// from the root node to the node. The depth of the node is the number of edges on the path from the
	/// root node to the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	///
	/// # Returns
	///
	/// The depth of the node in the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// assert_eq!(tree.get_node_depth(&node_3), 2);
	/// ```
	pub fn get_node_depth(&self, node_id: &Q) -> i32 {
		let node = self.get_node(node_id).unwrap();
		let mut depth = 0;
		let mut parent = node.get_parent();
		while parent.is_some() {
			depth += 1;
			parent = self.get_node(&parent.unwrap()).unwrap().get_parent();
		}
		depth
	}

	/// Get the height of the tree.
	///
	/// This method gets the height of the tree. The height of the tree is the length of the longest path
	/// from the root node to a leaf node. The height of the tree is the number of edges on the longest
	/// path from the root node to a leaf node.
	///
	/// # Returns
	///
	/// The height of the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// assert_eq!(tree.get_height(), 2);
	/// ```
	pub fn get_height(&self) -> i32 {
		let root = self.get_root_node().unwrap();
		self.get_node_height(&root.get_node_id())
	}

	/// Get the degree of a node in the tree.
	///
	/// This method gets the degree of a node in the tree. The degree of a node is the number of children
	/// that the node has.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	///
	/// # Returns
	///
	/// The degree of the node in the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
	///
	/// assert_eq!(tree.get_node_degree(&node_1), 2);
	/// assert_eq!(tree.get_node_degree(&node_2), 0);
	/// assert_eq!(tree.get_node_degree(&node_3), 0);
	/// ```
	pub fn get_node_degree(&self, node_id: &Q) -> i32 {
		let node = self.get_node(node_id).unwrap();
		node.get_children().len() as i32
	}

	/// Get the nodes in the tree.
	///
	/// This method gets the nodes in the tree.
	///
	/// # Returns
	///
	/// The nodes in the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node = Node::new(1, Some(2));
	/// tree.add_node(node.clone(), None).unwrap();
	///
	/// assert_eq!(tree.get_nodes().len(), 1);
	/// ```
	pub fn get_nodes(&self) -> &Vec<Node<Q, T>> {
		self.nodes.as_ref()
	}

	/// Remove a node from the tree.
	///
	/// This method removes a node from the tree. The node is removed using the given removal strategy.
	/// The removal strategy determines how the node and its children are removed from the tree. The
	/// `RetainChildren` strategy retains the children of the node when the node is removed. The
	/// `RemoveNodeAndChildren` strategy removes the node and its children when the node is removed.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to remove.
	/// * `strategy` - The strategy to use when removing the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree, NodeRemovalStrategy};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// tree.remove_node(&node_2, NodeRemovalStrategy::RetainChildren).unwrap();
	/// assert_eq!(tree.get_nodes().len(), 2);
	pub fn remove_node(&mut self, node_id: &Q, strategy: NodeRemovalStrategy) -> crate::prelude::Result<()> {
		match strategy {
			NodeRemovalStrategy::RetainChildren => {
				let node = self.get_node(node_id).unwrap();
				if let Some(parent_node_id) = &node.get_parent() {
					let parent_node = self.get_node(parent_node_id).unwrap();
					parent_node.remove_child(node.clone());
					let children = node.get_children();
					for child in children {
						parent_node.add_child(self.get_node(&child).unwrap());
					}
					self.nodes.retain(|n| &n.get_node_id() != node_id);
				} else {
					return Err(InvalidOperation("Cannot remove root node with RetainChildren strategy".to_string()));
				}
				Ok(())
			}
			NodeRemovalStrategy::RemoveNodeAndChildren => {
				let node = self.get_node(node_id).unwrap();
				let children = node.get_children();
				if let Some(parent_id) = node.get_parent() {
					let parent = self.get_node(&parent_id).unwrap();
					parent.remove_child(node.clone());
				}
				self.nodes.retain(|n| &n.get_node_id() != node_id);
				for child in children {
					let child = self.get_node(&child).unwrap();
					node.remove_child(child.clone());
					self.remove_node(&child.get_node_id(), strategy)?;
				}
				Ok(())
			}
		}
	}

	/// Get a subsection of the tree.
	///
	/// This method gets a subsection of the tree starting from the node with the given node id. The
	/// subsection is a list of nodes that are descendants of the node with the given node id upto the
	/// given number of descendants. If the number of descendants is `None`, all the descendants of the
	/// node are included in the subsection.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to get the subsection from.
	/// * `generations` - The number of descendants to include in the subsection. If `None`, all the
	/// descendants of the node are included in the subsection.
	///
	/// # Returns
	///
	/// The subsection of the tree starting from the node with the given node id.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree};
	///
	/// # let mut tree: Tree<i32, i32> = Tree::new();
	///
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// let subsection = tree.get_subtree(&node_2, None);
	/// assert_eq!(subsection.get_nodes().len(), 2);
	/// ```
	pub fn get_subtree(&self, node_id: &Q, generations: Option<i32>) -> SubTree<Q, T> {
		let mut subsection = Vec::new();
		if let Some(node) = self.get_node(node_id) {
			subsection.push(node.clone());
			// Get the subsequent children of the node recursively for the number of generations and add them to the subsection.
			if let Some(generations) = generations {
				let children = node.get_children();
				for current_generation in 0..generations {
					for child in children.clone() {
						subsection.append(
							&mut self
								.get_subtree(&child, Some(current_generation))
								.get_nodes().clone(),
						);
					}
				}
			} else {
				let children = node.get_children();
				for child in children {
					subsection.append(&mut self.get_subtree(&child, None).get_nodes().clone());
				}
			}
		}

		SubTree { nodes: subsection }
	}

	/// Add a subsection to the tree.
	///
	/// This method adds a subsection to the tree. The subsection is a list of nodes that are descendants
	/// of the node with the given node id. The subsection is added as children of the node with the
	/// given node id.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node to add the subsection to.
	/// * `subtree` - The subsection to add to the tree.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree, SubTree};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let mut subtree = SubTree::new();
	/// let node_2 = subtree.add_node(Node::new(2, Some(3)), None).unwrap();
	/// subtree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	/// tree.add_subtree(&node_id, subtree);
	/// assert_eq!(tree.get_nodes().len(), 3);
	/// ```
	pub fn add_subtree(&mut self, node_id: &Q, subtree: SubTree<Q, T>) {
		let node = self.get_node(node_id).unwrap();
		// Get the root node in the subsection and add it as a child of the node.
		let subtree_nodes = subtree.get_nodes();
		let root_node = subtree.get_root_node().unwrap();
		node.add_child(root_node.clone());
		self.nodes.append(&mut subtree_nodes.clone());
	}

	/// Traverse the subtree from the given node.
	///
	/// This method traverses the subtree from the given node in the given order.
	///
	/// # Arguments
	///
	/// * `order` - The order to traverse the tree.
	/// * `node_id` - The id of the node to start the traversal from.
	///
	/// # Returns
	///
	/// The nodes in the tree in the given order.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::{Node, Tree, TraversalStrategy};
	///
	/// let mut tree: Tree<i32, i32> = Tree::new();
	/// let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
	/// let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
	/// let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
	///
	/// let ordered_nodes = tree.traverse(TraversalStrategy::PreOrder, &node_1);
	/// # let expected = vec![1, 2, 3];
	/// # assert_eq!(ordered_nodes, expected);
	/// ```
	pub fn traverse(&self, order: TraversalStrategy, node_id: &Q) -> Vec<Q> {
		let mut nodes = vec![];
		let node = self.get_node(node_id).unwrap();
		match &order {
			TraversalStrategy::PreOrder => {
				nodes.push(node_id.clone());
				for child_id in node.get_children().iter() {
					nodes.append(&mut self.traverse(order, child_id));
				}
			}
			TraversalStrategy::PostOrder => {
				for child_id in node.get_children().iter() {
					nodes.append(&mut self.traverse(order, child_id));
				}
				nodes.push(node_id.clone());
			}
			TraversalStrategy::InOrder => {
				for (index, child_id) in node.get_children().iter().enumerate() {
					if index == 0 {
						nodes.append(&mut self.traverse(order, child_id));
						if !nodes.contains(child_id) {
							nodes.push(child_id.clone());
						}
						if !nodes.contains(node_id) {
							nodes.push(node_id.clone());
						}
					} else {
						nodes.push(child_id.clone());
						nodes.append(&mut self.traverse(order, child_id));
					}
				}
			}
		}
		let mut seen = HashSet::new();
		nodes.retain(|x| seen.insert(x.clone()));
		nodes
	}

	/// Print the tree.
	///
	/// This method prints the tree to the standard output.
	fn print_tree(
		tree: &Tree<Q, T>,
		f: &mut std::fmt::Formatter<'_>,
		node: &Node<Q, T>,
		level: usize,
		mut is_within: (bool, usize),
		is_last_child: bool,
	) -> std::fmt::Result
		where
			Q: PartialEq + Eq + Clone + Display + Hash,
			T: PartialEq + Eq + Clone + Display + Default,
	{
		for x in 1..level {
			if is_within.0 && x == is_within.1 {
				write!(f, "│   ")?;
			} else {
				write!(f, "    ")?;
			}
		}
		if level > 0 {
			if is_last_child {
				writeln!(f, "└── {}", node)?;
			} else {
				writeln!(f, "├── {}", node)?;
			}
		} else {
			writeln!(f, "{}", node)?;
		}
		let children = node.get_children();
		let children_count = children.len();
		for (index, child) in children.iter().enumerate() {
			let child = tree.get_node(child).unwrap();
			let last_item = index == children_count - 1;
			// Check if parent was last child
			let is_parent_last_item = if let Some(parent) = node.get_parent() {
				let parent = tree.get_node(&parent).unwrap();
				parent.get_children().last().unwrap() == &node.get_node_id()
			} else {
				true
			};
			if !is_within.0 {
				is_within.0 = !is_parent_last_item;
				is_within.1 = level;
			} else {
				is_within.1 = if level > 1 && level <= 3 { level - 1 } else if level > 3 { level - 2 } else { level };
			}
			Tree::print_tree(tree, f, &child, level + 1, (is_within.0, is_within.1), last_item)?;
		}
		Ok(())
	}
}

impl<Q, T> Default for Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone,
		T: PartialEq + Eq + Clone,
{
	fn default() -> Self {
		Tree { nodes: Vec::new() }
	}
}

impl<Q, T> Display for Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone + Display + Hash,
		T: PartialEq + Eq + Clone + Display + Default,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(node) = self.get_root_node() {
			Tree::print_tree(self, f, &node, 0, (false, 0), true)?;
		} else {
			let root = self.nodes.first().unwrap();
			Tree::print_tree(self, f, root, 0, (false, 0), true)?;
		}
		Ok(())
	}
}

impl<Q, T> Drop for Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone,
		T: PartialEq + Eq + Clone,
{
	fn drop(&mut self) {
		self.nodes.clear();
	}
}

#[cfg(feature = "serde")]
impl<Q, T> Serialize for Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone + Serialize,
		T: PartialEq + Eq + Clone + Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
	{
		let mut s = serializer.serialize_struct("Tree", 1)?;
		s.serialize_field("nodes", &self.nodes)?;
		s.end()
	}
}

#[cfg(feature = "serde")]
impl<'de, Q, T> Deserialize<'de> for Tree<Q, T>
	where
		Q: PartialEq + Eq + Clone + Deserialize<'de>,
		T: PartialEq + Eq + Clone + Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct TreeHelper<Q, T>
			where
				Q: PartialEq + Eq + Clone,
				T: PartialEq + Eq + Clone,
		{
			nodes: Vec<Node<Q, T>>,
		}

		let tree_helper = TreeHelper::deserialize(deserializer)?;
		Ok(Tree {
			nodes: tree_helper.nodes,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tree_new() {
		let tree = Tree::<u32, u32>::new();
		assert_eq!(tree.nodes.len(), 0);
	}

	#[test]
	fn test_tree_add_node() {
		let mut tree = Tree::new();
		let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		assert_eq!(tree.nodes.len(), 1);
		assert_eq!(node_id, 1);
		let node_id_2 = tree.add_node(Node::new(2, Some(3)), Some(&1)).unwrap();
		assert_eq!(tree.nodes.len(), 2);
		assert_eq!(node_id_2, 2);
		let node_2 = tree.get_node(&2).unwrap();
		assert_eq!(node_2.get_parent().unwrap(), 1);
	}

	#[test]
	fn test_tree_get_node() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		assert_eq!(tree.get_node(&1), Some(node));
		assert_eq!(tree.get_node(&2), None);
	}

	#[test]
	fn test_tree_get_nodes() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		assert_eq!(tree.get_nodes().len(), 1);
	}

	#[test]
	fn test_tree_get_root_node() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		assert_eq!(tree.get_root_node(), Some(node));
	}

	#[test]
	fn test_tree_get_node_height() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		assert_eq!(tree.get_node_height(&node_1), 2);
		assert_eq!(tree.get_node_height(&node_2), 1);
		assert_eq!(tree.get_node_height(&node_3), 0);
	}

	#[test]
	fn test_tree_get_node_depth() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		assert_eq!(tree.get_node_depth(&node_3), 2);
		assert_eq!(tree.get_node_depth(&node_2), 1);
		assert_eq!(tree.get_node_depth(&node_1), 0);
	}

	#[test]
	fn test_tree_get_height() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		assert_eq!(tree.get_height(), 2);
	}

	#[test]
	fn test_tree_get_node_degree() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
		assert_eq!(tree.get_node_degree(&node_1), 2);
		assert_eq!(tree.get_node_degree(&node_2), 0);
		assert_eq!(tree.get_node_degree(&node_3), 0);
	}

	#[test]
	fn test_tree_remove_node() -> crate::prelude::Result<()> {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None)?;
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(&1))?;
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(&2))?;
		tree.remove_node(&2, NodeRemovalStrategy::RetainChildren)?;
		assert_eq!(tree.get_nodes().len(), 2);
		let node_4 = Node::new(4, Some(5));
		let node_5 = Node::new(5, Some(12));
		tree.add_node(node_4.clone(), Some(&3))?;
		tree.add_node(node_5.clone(), Some(&3))?;
		tree.remove_node(&3, NodeRemovalStrategy::RemoveNodeAndChildren)?;
		assert_eq!(tree.get_nodes().len(), 1);
		Ok(())
	}

	#[test]
	fn test_tree_get_subsection() {
		let mut tree = Tree::new();
		let node = Node::new(1, Some(2));
		tree.add_node(node.clone(), None).unwrap();
		let node_2 = Node::new(2, Some(3));
		tree.add_node(node_2.clone(), Some(&1)).unwrap();
		let node_3 = Node::new(3, Some(6));
		tree.add_node(node_3.clone(), Some(&2)).unwrap();
		let node_4 = Node::new(4, Some(5));
		tree.add_node(node_4.clone(), Some(&2)).unwrap();
		let node_5 = Node::new(5, Some(6));
		tree.add_node(node_5.clone(), Some(&3)).unwrap();
		let subsection = tree.get_subtree(&2, None);
		assert_eq!(subsection.get_nodes().len(), 4);
		let subsection = tree.get_subtree(&2, Some(0));
		assert_eq!(subsection.get_nodes().len(), 1);
		let subsection = tree.get_subtree(&2, Some(1));
		assert_eq!(subsection.get_nodes().len(), 3);
	}

	#[test]
	fn test_tree_add_subsection() {
		let mut tree = Tree::new();
		let node_id = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let mut subtree = SubTree::new();
		let node_2 = subtree.add_node(Node::new(2, Some(3)), None).unwrap();
		subtree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		tree.add_subtree(&node_id, subtree);
		assert_eq!(tree.get_nodes().len(), 3);
	}

	#[test]
	fn test_tree_display() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
		let expected_str = "1: 2\n└── 2: 3\n    ├── 3: 6\n    │   └── 5: 6\n    └── 4: 5\n";
		assert_eq!(tree.to_string(), expected_str);
	}

	#[test]
	fn compare_tree() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
		let mut tree_2 = Tree::new();
		let node_1 = tree_2.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree_2.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree_2.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		tree_2.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
		tree_2.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
		assert_eq!(tree, tree_2);
	}

	#[test]
	fn test_tree_traverse() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_1)).unwrap();
		let node_4 = tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
		let node_5 = tree.add_node(Node::new(5, Some(6)), Some(&node_2)).unwrap();
		let node_6 = tree.add_node(Node::new(6, Some(7)), Some(&node_3)).unwrap();
		let preorder_nodes = tree.traverse(TraversalStrategy::PreOrder, &node_1);
		let expected_preorder = vec![node_1, node_2, node_4, node_5, node_3, node_6];
		assert_eq!(preorder_nodes, expected_preorder);

		let in_order_nodes = tree.traverse(TraversalStrategy::InOrder, &node_1);
		let expected_in_order = vec![node_4, node_2, node_5, node_1, node_3, node_6];
		assert_eq!(in_order_nodes, expected_in_order);

		let post_order_nodes = tree.traverse(TraversalStrategy::PostOrder, &node_1);
		let expected_post_order = vec![node_4, node_5, node_2, node_6, node_3, node_1];
		assert_eq!(post_order_nodes, expected_post_order);
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_tree_serialize_and_deserialize() {
		let mut tree = Tree::new();
		let node_1 = tree.add_node(Node::new(1, Some(2)), None).unwrap();
		let node_2 = tree.add_node(Node::new(2, Some(3)), Some(&node_1)).unwrap();
		let node_3 = tree.add_node(Node::new(3, Some(6)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(4, Some(5)), Some(&node_2)).unwrap();
		tree.add_node(Node::new(5, Some(6)), Some(&node_3)).unwrap();
		let serialized = serde_json::to_string(&tree).unwrap();
		let expected = r#"{"nodes":[{"node_id":1,"value":2,"parent":null,"children":[2]},{"node_id":2,"value":3,"parent":1,"children":[3,4]},{"node_id":3,"value":6,"parent":2,"children":[5]},{"node_id":4,"value":5,"parent":2,"children":[]},{"node_id":5,"value":6,"parent":3,"children":[]}]}"#;
		let deserialized: Tree<u32, u32> = serde_json::from_str(&serialized).unwrap();
		let expected_tree: Tree<u32, u32> = serde_json::from_str(expected).unwrap();
		assert_eq!(deserialized, expected_tree);
	}
}
