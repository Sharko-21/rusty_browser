use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::{RefCell};
use std::ops::Add;
use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
pub struct Node {
    children: Vec<Rc<RefCell<Node>>>,
    node_type: NodeType
}

impl Node {
    pub fn new(parent: &mut DomElem, node_type: NodeType) -> Rc<RefCell<Node>> {
        let node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node{children: vec![], node_type}));
        match parent {
            DomElem::Node(parent) => parent.borrow_mut().children.push(node.clone()),
            DomElem::None => {}
        }
        return node
    }

    pub fn get_type(&self) -> & NodeType {
        &self.node_type
    }

    pub fn get_children(&self) -> & Vec<Rc<RefCell<Node>>> {
        &self.children
    }

    pub fn set_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }

    pub fn print_subtree_by_name(&self) -> String {
        let mut dom: String = String::new();
        dom = dom + &format!("<{}>", &self.node_type.to_string());
        for child in &self.children {
            dom = dom + &child.borrow().print_subtree_by_name();
        }
        dom = dom + &format!("</{}>", &self.node_type.to_string());
        dom
    }
}

pub enum DomElem {
    Node(Rc<RefCell<Node>>),
    None
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Text(text) => write!(f,  "{}", text.to_string()),
            NodeType::Element(element) => write!(f, "{}", element)
        }
    }
}

#[derive(Debug)]
pub enum TAG {
    HTML,
    HEAD,
    BODY,
    P,
    DIV
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: TAG,
    pub attributes: Attributes,
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut print_body: String = String::new();
        print_body = print_body + &format!("{:?}", &self.tag_name);
        for (key, value) in &self.attributes {
            print_body = print_body + &format!(" {}=\"{}\" ", key, value);
        }
        write!(f, "{}", print_body)
    }
}

type Attributes = HashMap<String, String>;
