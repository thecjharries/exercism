pub mod graph {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<&'static str, &'static str>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq)]
            pub struct Edge {
                pair: (String, String),
                attrs: HashMap<&'static str, &'static str>,
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

            #[derive(Debug, PartialEq)]
            pub struct Node {
                title: String,
                attrs: HashMap<&'static str, &'static str>,
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
