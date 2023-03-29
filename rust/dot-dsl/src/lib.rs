pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(&self, nodes: &[graph_items::node::Node]) -> Self {
            let nodes = nodes.to_vec();
            let mut edges = Vec::new();
            for edge in self.edges.iter() {
                edges.push(edge.clone());
            }
            let mut attrs = HashMap::new();
            while let Some((k, v)) = self.attrs.iter().next() {
                attrs.insert(k.clone(), v.clone());
            }
            Graph {
                nodes,
                edges,
                attrs,
            }
        }

        pub fn with_edges(&self, edges: &[graph_items::edge::Edge]) -> Self {
            let mut nodes = Vec::new();
            for node in self.nodes.iter() {
                nodes.push(node.clone());
            }
            let edges = edges.to_vec();
            let mut attrs = HashMap::new();
            while let Some((k, v)) = self.attrs.iter().next() {
                attrs.insert(k.clone(), v.clone());
            }
            Graph {
                nodes,
                edges,
                attrs,
            }
        }

        pub fn node(&self, title: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|n| n.title == title)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub pair: (String, String),
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        pair: (a.to_string(), b.to_string()),
                        attrs: HashMap::new(),
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                pub title: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(a: &str) -> Self {
                    Node {
                        title: a.to_string(),
                        attrs: HashMap::new(),
                    }
                }
            }
        }
    }
}
