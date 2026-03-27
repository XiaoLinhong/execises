
pub mod graph {
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            use super::node::Node;
            #[derive(PartialEq, Clone, Debug)]
            pub struct Edge{
                pub points: [Node; 2],
                // attrs: Vec<(String, String)>,
                pub attrs: HashMap<String, String>
            }

            impl Edge {
                pub fn new(p1: &str, p2: &str) -> Self {
                    Self {
                        points: [Node::new(p1), Node::new(p2)],
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(self, new_attrs: &[(&str, &str)] ) -> Self{
                    let mut attrs = self.attrs;
                    attrs.extend(new_attrs.iter().map(|(name, attr)| (name.to_string(), attr.to_string())));
                    Self{
                        attrs: attrs,
                        ..self
                    }
                }

                pub fn attr(&self, k: &str) -> Option<&str>{
                    self.attrs.get(k).map(|s| &s[..])
                }
            }
            
        }

         pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Clone, Debug)]
            pub struct Node{
                pub name: String,
                // attrs: Vec<(String, String)>,
                pub attrs: HashMap<String, String>

            }

            impl Node {
                pub fn new(s: &str) -> Self {
                    Self {
                        name: s.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(self, new_attrs: &[(&str, &str)] ) -> Self{
                    let mut attrs = self.attrs;
                    attrs.extend(new_attrs.iter().map(|(name, attr)| (name.to_string(), attr.to_string())));
                    Self{
                        attrs: attrs,
                        ..self
                    }
                }
                pub fn attr(&self, k: &str) -> Option<&str>{
                    self.attrs.get(k).map(|s| &s[..])
                }
            }
            
        }
    }

    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph{
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        // pub attrs: Vec<(String, String)>,
        pub attrs: HashMap<String, String>

    }

    impl Graph {
        pub fn new() -> Self {
            Self{
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, new_nodes: &[Node]) -> Self {
            let mut nodes = self.nodes;
            nodes.extend_from_slice(new_nodes);
            Self{
                nodes: nodes,
                ..self
            }
        }

        pub fn with_edges(self, new_edges: &[Edge]) -> Self {
            let mut edges = self.edges;
            edges.extend_from_slice(new_edges);
            Self{
                edges: edges,
                ..self
            }
        }

        pub fn with_attrs(self, new_attrs: &[(&str, &str)] ) -> Self {
            let mut attrs = self.attrs;
            attrs.extend(new_attrs.iter().map(|(name, attr)| (name.to_string(), attr.to_string())));
            Self{
                attrs: attrs,
                ..self
            }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    
    }
}
