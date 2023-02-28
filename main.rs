use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    id: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Edge {
    from: Node,
    to: Node,
    weight: i32,
}

impl Edge {
    fn new(from: Node, to: Node, weight: i32) -> Edge {
        Edge { from, to, weight }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge);
    }

    fn get_neighbors(&self, node: &Node) -> HashSet<Node> {
        let mut neighbors = HashSet::new();
        for edge in &self.edges {
            if &edge.from == node {
                neighbors.insert(edge.to);
            } else if &edge.to == node {
                neighbors.insert(edge.from);
            }
        }
        neighbors
    }
}

fn find(parents: &mut HashMap<Node, Node>, node: Node) -> Node {
    let mut curr = node;
    while parents[&curr] != curr {
        let parent = parents[&curr];
        curr = parent;
    }
    parents.entry(node).and_modify(|e| *e = curr);
    curr
}

fn kruskal(graph: &Graph) -> Vec<Edge> {
    let mut result = Vec::new();
    let mut parents = HashMap::new();
    for node in &graph.nodes {
        parents.insert(*node, *node);
    }
    let mut sorted_graph = graph.edges.clone();
    sorted_graph.sort();
    for edge in sorted_graph {
        let parent1 = find(&mut parents, edge.from);
        let parent2 = find(&mut parents, edge.to);
        if parent1 != parent2 {
            result.push(edge);
            parents.insert(parent1, parent2);
        }
    }
    result
}

fn read_graph(file_path: &Path) -> Graph {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut graph = Graph::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let from: Node = Node {
            id: parts[0].parse().unwrap(),
        };
        let to: Node = Node {
            id: parts[1].parse().unwrap(),
        };
        let weight = parts[2].parse().unwrap();
        graph.add_node(from);
        graph.add_node(to);
        graph.add_edge(Edge::new(from, to, weight));
    }
    graph
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(&args[1]);
    let graph = read_graph(file_path);
    let minimum_spanning_tree = kruskal(&graph);

    for edge in &minimum_spanning_tree {
        println!("{}", edge);
    }
}