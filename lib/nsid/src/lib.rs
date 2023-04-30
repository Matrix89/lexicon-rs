use std::collections::HashMap;

use lexicon::lexicon::UserType;

#[derive(Debug, Clone)]
pub enum NSIDNode {
    Segment {
        name: String,
        children: Vec<NSIDNode>,
    },
    Identifier {
        name: String,
        def: HashMap<String, UserType>,
    },
}

impl NSIDNode {
    pub fn root() -> Self {
        NSIDNode::Segment {
            name: "lexicon".to_string(),
            children: vec![],
        }
    }

    pub fn add(&mut self, nsid: &str, def: HashMap<String, UserType>) {
        let parts = nsid.splitn(2, '.').collect::<Vec<&str>>();
        match self {
            NSIDNode::Segment { name, children } => {
                if parts.len() == 1 {
                    let identifier = NSIDNode::Identifier {
                        name: parts[0].to_string(),
                        def,
                    };

                    children.push(identifier);
                    return;
                }

                if name == parts[0] {
                    if parts.len() == 1 {
                        panic!("Identifier mismatch: {} != {}", name, parts[0]);
                    }

                    if parts.len() > 2 {
                        panic!("Too many parts: {}", nsid);
                    }

                    let identifier = NSIDNode::Identifier {
                        name: parts[1].to_string(),
                        def,
                    };

                    children.push(identifier);
                } else {
                    if parts.len() == 1 {
                        let identifier = NSIDNode::Identifier {
                            name: parts[0].to_string(),
                            def,
                        };

                        children.push(identifier);
                        return;
                    }

                    if let Some(segment) = children.iter_mut().find_map(|child| match child {
                        NSIDNode::Segment { name, .. } => {
                            if name == parts[0] {
                                Some(child)
                            } else {
                                None
                            }
                        }
                        NSIDNode::Identifier { .. } => None,
                    }) {
                        segment.add(&parts[1..].join("."), def);
                    } else {
                        let mut segment = NSIDNode::Segment {
                            name: parts[0].to_string(),
                            children: vec![],
                        };
                        segment.add(&parts[1..].join("."), def);
                        children.push(segment);
                    };
                }
            }
            NSIDNode::Identifier { name, .. } => {
                if name != parts[0] {
                    panic!("Identifier mismatch: {} != {}", name, parts[0]);
                }
            }
        }
    }

    fn find_impl(self: &NSIDNode, path: Vec<&str>, ident: &str) -> Option<UserType> {
        match self {
            NSIDNode::Segment { name, children } => {
                let child = children.iter().find(|child| match child {
                    NSIDNode::Segment {
                        name: child_name, ..
                    } => child_name == path[0],
                    NSIDNode::Identifier { name, .. } => name == path[0],
                });

                return child?.find_impl(path[1..].to_vec(), ident);
            }
            NSIDNode::Identifier { name, def } => {
                return def.get(ident).cloned();
            }
        }
    }

    pub fn find(self: &NSIDNode, path: &String) -> Option<UserType> {
        let mut namespace_ident = path.split("#");
        let namespace = namespace_ident
            .next()
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>();
        let ident = namespace_ident.next().unwrap_or("main");
        return self.find_impl(namespace, ident);
    }
}
