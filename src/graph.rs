use std::collections::HashMap;

type NodeId = usize;

pub struct WordGraph {
	nodes: Vec<Node>,
}

pub struct Node {
	data: char,
	children: Option<Vec<NodeId>>,
}

impl WordGraph {
	pub fn add_word(&mut self, word: String) {
		let char_vec: Vec<char> = word.chars().collect();

		let mut next: &Node;

		for (i, c) in word.chars().enumerate() {
			if i == 0 {
				let (contains, index) = self.contains_char(c);

				if contains {
					next = &self.nodes[index];
				} else {
					next = &Node {
						data: c,
						children: None,
					}
				}
			} else {
				// check if child exists
				// if not, create new vec with child
				// else, assign next to be that child, keep going
			}
		}
	}

	fn contains_char(&self, c: char) -> (bool, NodeId) {
		for (i, node) in self.nodes.iter().enumerate() {
			if node.data == c {
				return (true, i);
			}
		}

		return (false, 0);
	}

	pub fn contains_word(&self, word: String) -> bool {
		unimplemented!();
	}
}
