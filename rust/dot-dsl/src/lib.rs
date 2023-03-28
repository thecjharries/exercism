use std::collections::HashMap;

pub mod graph {
    #[derive(Debug, PartialEq)]
    pub struct Graph {
        nodes: Vec<graph_items::node::Node>,
        edges: Vec<graph_items::edge::Edge>,
        attrs: HashMap<&'static str, &'static str>,
    };

    impl Graph {
        pub fn new() -> Self {
            Graph {}
        }
    }

    pub mod graph_items {
        pub mod edge {
            #[derive(Debug, PartialEq)]
            pub struct Edge {
                pair: (String, String),
                attrs: HashMap<&'static str, &'static str>,
            };

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {}
                }
            }
        }

        pub mod node {
            #[derive(Debug, PartialEq)]
            pub struct Node {
                title: String,
                attrs: HashMap<&'static str, &'static str>,
            };

            impl Node {
                pub fn new(a: &str) -> Self {
                    Node {}
                }
            }
        }
    }
}
