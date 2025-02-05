use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::NodeRef;
use std::fmt;
use std::fs::write;
use std::process::Command;

fn dot_to_svg(dot: &str, output_path: &str) {
    let dot_file = "graph.dot";
    write(dot_file, dot).expect("Failed to write DOT file");

    let output = Command::new("dot")
        .args(&["-Tsvg", dot_file, "-o", output_path])
        .output()
        .expect("Failed to execute Graphviz");

    if output.status.success() {
        println!("SVG generated at {}", output_path);
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}

// Define a struct for nodes
#[derive(Debug, Clone, Copy)]
struct NodeData {
    label: &'static str,
    shape: &'static str, // "rectangle" or "circle"
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label={}, shape={}", self.label, self.shape)
    }
}


fn main() {
    let mut graph = DiGraph::<NodeData, &str>::new();

    // Add nodes with proper labels and shapes
    let a = graph.add_node(NodeData {
        label: "Start",
        shape: "rectangle",
    });
    let b = graph.add_node(NodeData {
        label: "Process",
        shape: "rectangle",
    });
    let c = graph.add_node(NodeData {
        label: "Decision",
        shape: "circle",
    });
    let d = graph.add_node(NodeData {
        label: "End",
        shape: "circle",
    });

    // Add edges
    graph.add_edge(a, b, "Next");
    graph.add_edge(b, c, "Check");
    graph.add_edge(c, d, "Done");

    // Custom attribute getter for nodes
    let get_node_attrs = |_, node: (NodeIndex, &NodeData)| {
        format!("label=\"{}\" shape={}", node.1.label, node.1.shape)
    };

    // Custom attribute getter for edges
    let get_edge_attrs = |_, _| {
        String::new() // No extra edge attributes
    };

    // Generate proper Graphviz DOT format
    let dot_graph = Dot::with_attr_getters(
        &graph,
        &[Config::EdgeNoLabel],
        &get_edge_attrs,
        &get_node_attrs,
    )
    .to_string();
    println!("{}", dot_graph);
    dot_to_svg(&dot_graph, "graph.svg");
}
