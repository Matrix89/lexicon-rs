use std::collections::HashMap;

use lexicon::lexicon::UserType;

#[derive(Debug)]
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
}
