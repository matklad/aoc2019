use std::{
    collections::HashMap,
    io::{stdin, Read},
    ops,
};

fn main() -> aoc::Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let tree = parse(&buf);
    println!("{}", calculate_orbits(&tree));
    Ok(())
}

fn calculate_orbits(tree: &Tree) -> u64 {
    let (_size, orbits) = go(tree, tree.root());
    orbits
}

fn go(tree: &Tree, node: NodeId) -> (u64, u64) // (size, orbits)
{
    let mut subtree_size = 1;
    let mut subtree_orbits = 0;
    for &child in tree[node].children.iter() {
        let (size, orbits) = go(tree, child);
        subtree_size += size;
        subtree_orbits += orbits + size;
    }
    (subtree_size, subtree_orbits)
}

fn parse(text: &str) -> Tree {
    let mut ids = HashMap::new();

    let mut tree = Tree::default();
    ids.insert("COM", tree.alloc());

    for line in text.trim().lines() {
        let idx = line.find(')').unwrap();
        let parent = &line[..idx];
        let child = &line[idx + 1..];
        let parent = *ids.entry(parent).or_insert_with(|| tree.alloc());
        let child = *ids.entry(child).or_insert_with(|| tree.alloc());
        tree[parent].children.push(child);
    }

    tree
}

#[derive(Default)]
struct Tree {
    nodes: Vec<Node>,
}

#[derive(Clone, Copy)]
struct NodeId(u32);

#[derive(Default)]
struct Node {
    children: Vec<NodeId>,
}

impl Tree {
    fn alloc(&mut self) -> NodeId {
        self.nodes.push(Node::default());
        NodeId((self.nodes.len() - 1) as u32)
    }

    fn root(&self) -> NodeId {
        assert!(!self.nodes.is_empty());
        NodeId(0)
    }
}

impl ops::Index<NodeId> for Tree {
    type Output = Node;
    fn index(&self, index: NodeId) -> &Self::Output {
        &self.nodes[index.0 as usize]
    }
}

impl ops::IndexMut<NodeId> for Tree {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.nodes[index.0 as usize]
    }
}

#[test]
fn test_examples() {
    let tree = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    assert_eq!(calculate_orbits(&parse(tree)), 42)
}
