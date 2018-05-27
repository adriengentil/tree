#![feature(test)]
extern crate test;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::io::BufReader;

struct Node {
    label: String,
    id: i64,
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// HashMap key is the parent_id, the value is the list of children
// The children are reprensented by a HashSet in order to eliminate
// potential duplicates in the input file :'(
fn build_tree_from_file(filename: String) -> HashMap<i64, HashSet<Node>> {
    let mut tree = HashMap::new();

    let f = File::open(filename).expect("File not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        // file format: ID, LABEL, PARENT_ID
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }

        let s: Vec<&str> = l.split(',').collect();
        let id = s[0].to_string().parse::<i64>().expect("Invalid ID");
        let parent_id = s[2].to_string().parse::<i64>().unwrap_or(0);

        let node = Node {
            label: s[1].to_string(),
            id: id,
        };
        let node_vec = tree.entry(parent_id).or_insert(HashSet::new());
        node_vec.insert(node);
    }

    tree
}

// return only paths root from leaf
// recursive version
fn compute_path_from_tree(
    tree: &HashMap<i64, HashSet<Node>>,
    root_id: i64,
    prefix: String,
) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();

    match tree.get(&root_id) {
        None => path,
        Some(root) => {
            for node in root {
                let prefix = format!("{}/{}", prefix, node.label);
                let child_path_vec = compute_path_from_tree(&tree, node.id, prefix.clone());

                if child_path_vec.is_empty() {
                    // no children == leaf
                    path.push(prefix);
                } else {
                    path.extend(child_path_vec);
                }
            }
            path
        }
    }
}

// iterative version
fn compute_path_from_tree_iterative(tree: &HashMap<i64, HashSet<Node>>) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();

    let root_id: i64 = 0;
    let root_node = tree.get(&root_id).expect("Root node not found");
    let mut node_list: Vec<(String, &Node)> = Vec::new();

    for node in root_node {
        let prefix = format!("/{}", node.label);
        node_list.push((prefix, &node));
    }

    while let Some(ctx) = node_list.pop() {
        let children = tree.get(&ctx.1.id);
        match children {
            None => {
                // no children == leaf
                path.push(ctx.0.clone());
            }
            Some(nodes) => {
                for node in nodes {
                    let prefix = format!("{}/{}", ctx.0, node.label);
                    node_list.push((prefix, &node));
                }
            }
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_build_tree_from_file(b: &mut Bencher) {
        b.iter(|| build_tree_from_file(String::from("atree")));
    }

    #[bench]
    fn bench_compute_path_from_tree_iterative(b: &mut Bencher) {
        let filename = "atree";
        let tree = build_tree_from_file(filename.to_string());

        b.iter(|| compute_path_from_tree_iterative(&tree));
    }

    #[bench]
    fn bench_compute_path_from_tree(b: &mut Bencher) {
        let filename = "atree";
        let tree = build_tree_from_file(filename.to_string());

        b.iter(|| compute_path_from_tree(&tree, 0, String::new()));
    }
}
