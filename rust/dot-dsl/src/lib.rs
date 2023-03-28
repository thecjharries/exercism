pub mod graph {
    pub struct Graph;

    impl Graph {
        pub fn new() -> Self {
            Graph {}
        }
    }

    pub mod graph_items {
        pub mod edge {
            pub struct Edge;

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {}
                }
            }
        }

        pub mod node {
            pub struct Node;

            impl Node {
                pub fn new(a: &str) -> Self {
                    Node {}
                }
            }
        }
    }
}
