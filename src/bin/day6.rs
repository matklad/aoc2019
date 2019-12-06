use std::{
    collections::HashMap,
    io::{stdin, Read},
    ops,
};

fn main() -> aoc::Result<()> {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf)?;
    let (tree, idx) = parse(&buf);
    let me = idx["YOU"];
    let santa = idx["SAN"];
    println!("{}", calculate_dist(&tree, me, santa) - 2);
    Ok(())
}

fn calculate_orbits(tree: &Tree) -> u64 {
    let (_size, orbits) = go(tree, tree.root());
    return orbits;

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
}

fn calculate_dist(tree: &Tree, u: NodeId, v: NodeId) -> u32 {
    match go(tree, u, v, tree.root()) {
        Dists::Both(it) => return it,
        _ => panic!("missing nodes in tree"),
    }

    #[derive(Clone, Copy)]
    enum Dists {
        None,
        One(u32),
        Both(u32),
    }

    impl Dists {
        fn is_both(&self) -> bool {
            match self {
                Dists::Both(_) => true,
                _ => false,
            }
        }
        fn merge(&mut self, other: Dists) {
            *self = match (*self, other) {
                (both @ Dists::Both(_), _) | (_, both @ Dists::Both(_)) => both,
                (Dists::None, other) | (other, Dists::None) => other,
                (Dists::One(d1), Dists::One(d2)) => Dists::Both(d1 + d2),
            }
        }
        fn increment(&mut self) {
            if let Dists::One(d) = self {
                *d += 1;
            }
        }
    }

    fn go(tree: &Tree, u: NodeId, v: NodeId, node: NodeId) -> Dists {
        let mut res = Dists::None;
        if node == u {
            res.merge(Dists::One(0))
        }
        if node == v {
            res.merge(Dists::One(0))
        }
        for &child in tree[node].children.iter() {
            if res.is_both() {
                break;
            }
            let mut d = go(tree, u, v, child);
            d.increment();
            res.merge(d)
        }
        res
    }
}

fn parse(text: &str) -> (Tree, HashMap<&str, NodeId>) {
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

    (tree, ids)
}

#[derive(Default)]
struct Tree {
    nodes: Vec<Node>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
fn test_example_orbits() {
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
    let (tree, _ids) = parse(tree);
    assert_eq!(calculate_orbits(&tree), 42)
}

#[test]
fn test_example_dists() {
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
K)L
K)YOU
I)SAN";
    let (tree, ids) = parse(tree);
    let me = ids["YOU"];
    let santa = ids["SAN"];
    assert_eq!(calculate_dist(&tree, me, santa) - 2, 4)
}
