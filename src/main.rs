use petgraph::dot::{Dot, Config};
use petgraph::graph::{DiGraph};
use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug)]
struct Node {
    data: f64,
    _op: Option<String>,
    _prev: Vec<usize>,
}

fn trace(root: usize, nodes: &Vec<Node>) -> (HashSet<usize>, Vec<(usize, usize)>) {
    let mut visited_nodes = HashSet::new();
    let mut edges = Vec::new();

    fn build(v: usize, nodes: &Vec<Node>, visited: &mut HashSet<usize>, edges: &mut Vec<(usize, usize)>) {
        if !visited.contains(&v) {
            visited.insert(v);
            for &child_idx in &nodes[v]._prev {
                edges.push((child_idx, v));
                build(child_idx, nodes, visited, edges);
            }
        }
    }

    build(root, nodes, &mut visited_nodes, &mut edges);
    (visited_nodes, edges)
}

fn draw_dot(root: usize, nodes: &Vec<Node>) {
    let (visited_nodes, edges) = trace(root, nodes);

    // Map for storing node IDs to Graphviz IDs
    let mut node_id_map = HashMap::new();
    let mut graph = DiGraph::<String, ()>::new();

    for &node_idx in &visited_nodes {
        let node = &nodes[node_idx];
        let node_label = format!("{{data {:.4}}}", node.data);
        let graph_node = graph.add_node(node_label.clone());
        node_id_map.insert(node_idx, graph_node);

        if let Some(op) = &node._op {
            let op_node = graph.add_node(op.clone());
            graph.add_edge(op_node, graph_node, ());
        }
    }

    for (from, to) in edges {
        if let Some(&from_id) = node_id_map.get(&from) {
            if let Some(_) = node_id_map.get(&to) {
                if let Some(op) = &nodes[to]._op {
                    let op_node = graph.add_node(op.clone());
                    graph.add_edge(from_id, op_node, ());
                }
            }
        }
    }

    // Print the DOT representation
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

fn main() {
    // Example usage
    let nodes = vec![
        Node {
            data: 1.0,
            _op: None,
            _prev: vec![],
        },
        Node {
            data: 3.0,
            _op: Some("add".to_string()),
            _prev: vec![0],
        },
    ];

    draw_dot(1, &nodes);
}

