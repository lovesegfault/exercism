pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Clone, Default, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_attrs<K, V>(mut self, attrs: &[(K, V)]) -> Self
        where
            K: AsRef<str>,
            V: AsRef<str>,
        {
            attrs.iter().for_each(|(k, v)| {
                self.attrs
                    .insert(k.as_ref().to_string(), v.as_ref().to_string());
            });
            self
        }

        pub fn get_node<S: AsRef<str>>(&self, name: S) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == name.as_ref()).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new<A, B>(a: A, b: B) -> Self
                where
                    A: AsRef<str>,
                    B: AsRef<str>,
                {
                    Self {
                        src: a.as_ref().to_string(),
                        dst: b.as_ref().to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<K, V>(mut self, attrs: &[(K, V)]) -> Self
                where
                    K: AsRef<str>,
                    V: AsRef<str>,
                {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs
                            .insert(k.as_ref().to_string(), v.as_ref().to_string());
                    });
                    self
                }
            }
        }
        pub mod node {
            use std::{borrow::Borrow, collections::HashMap};

            #[derive(Clone, PartialEq, Eq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new<S: AsRef<str>>(name: S) -> Self {
                    Self {
                        name: name.as_ref().to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<K, V>(mut self, attrs: &[(K, V)]) -> Self
                where
                    K: AsRef<str>,
                    V: AsRef<str>,
                {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs
                            .insert(k.as_ref().to_string(), v.as_ref().to_string());
                    });
                    self
                }

                pub fn get_attr<S: AsRef<str>>(&self, name: S) -> Option<&str> {
                    self.attrs.get(name.as_ref()).map(|s| s.borrow())
                }
            }
        }
    }
}
