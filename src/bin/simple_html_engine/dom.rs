use std::collections::{HashMap,HashSet};

pub type AttrMap = HashMap<String,String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type:NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]