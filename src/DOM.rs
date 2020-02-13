use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::ops::Add;
use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
pub struct Node {
    children: Vec<Rc<RefCell<Node>>>,
    dom: Rc<RefCell<DOM>>,
    node_type: NodeType
}

impl Node {
    pub fn new(parent: &mut DomElem, node_type: NodeType, dom: Rc<RefCell<DOM>>) -> Rc<RefCell<Node>> {
        let node: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node{children: vec![], dom, node_type}));
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

    pub fn set_attr(&mut self, self_rc: Rc<RefCell<Node>>, key: String, value: String) {
        if key == "class".to_string() {
            let mut dom_ref = self.dom.borrow_mut();
            match dom_ref.classes_node.get_mut(&key) {
                None => {
                    dom_ref.classes_node.insert(value.clone(), vec![self_rc.clone()]);
                },
                Some(vector) => {
                    vector.push(self_rc.clone());
                }
            }
        } else if key == "id".to_string() {
            self.dom.borrow_mut().id_node.insert(value.clone(), self_rc);
        }

        match &mut self.node_type {
            NodeType::Text(str) => {},
            NodeType::Element(element) => {
                element.attributes.insert(key, value);
            },
            _ => {}
        };
        return;
    }
}

#[derive(Debug)]
pub struct DOM {
    root_node: DomElem,
    classes_node: HashMap<String, Vec<Rc<RefCell<Node>>>>,
    id_node: HashMap<String, Rc<RefCell<Node>>>
}

impl DOM {
    pub fn new(root_node: DomElem) -> Rc<RefCell<DOM>>{
        Rc::new(RefCell::new(DOM{root_node, classes_node:HashMap::new(), id_node:HashMap::new()}))
    }

    pub fn set_root(&mut self, root_node: DomElem) {
        self.root_node = root_node;
    }

    pub fn find_by_class(&self, class: String) -> Option<&Vec<Rc<RefCell<Node>>>> {
        self.classes_node.get(&class)
    }

    pub fn find_by_id(&self, id: String) -> Option<&Rc<RefCell<Node>>> {
        self.id_node.get(&id)
    }

    pub fn print_tree(&self) -> String {
        match &self.root_node {
            DomElem::Node(node) => node.borrow().print_subtree_by_name(),
            _ => {
                String::new()
            }
        }
    }
}

#[derive(Debug)]
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
    tag_name: TAG,
    attributes: Attributes,
}

impl ElementData {
    pub fn new(tag_name: TAG, attributes: Attributes) -> ElementData {
        ElementData{tag_name, attributes}
    }
}

impl fmt::Display for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut print_body: String = String::new();
        print_body = print_body + &format!("{:?}", &self.tag_name);
        for (key, value) in &self.attributes {
            print_body = print_body + &format!(" {}=\"{}\"", key, value);
        }
        write!(f, "{}", print_body)
    }
}

type Attributes = HashMap<String, String>;
