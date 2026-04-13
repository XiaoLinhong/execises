use std::fmt::Debug;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    head: Option<Node<T>>, // 为了转移所有权，加了有个Option
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node<T> {
    label: T,
    children: Vec<Node<T>>,
}

impl <T: Debug + Ord> Node<T> {
    pub fn take(&mut self, from: &T, nodes: &mut Vec<Self>) -> bool{

        if self.children.is_empty() {
            return false;
        }

        // 移除子节点的所有权
        let mut idx = self.children.iter().position(|child|child.label == *from);

        // 遍历所有子节点的子节点是否满足
        if idx.is_none() {
            idx = self.children.iter_mut().position(|child| child.take(from, nodes));
        }

        if idx.is_some() {
            let child = self.children.remove(idx.unwrap());
            nodes.push(child);
            return true;
        }
        false
    }

    fn find_path<'a>(&'a self, to: &T, path: &mut Vec<&'a T>) -> bool{
        if self.label == *to {
            path.push(&self.label);
            return true;
        }

        if !self.children.is_empty() {
            for child in &self.children { // 遍历所有子节点
                if !child.find_path(to, path) {
                    path.pop();
                } else {
                    path.push(&self.label);
                    return true;
                }
            }
        }
        false
    }
    
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self{head: Some(Node {label, children: Vec::new() })}
    }
    /// Builder-method for constructing a tree with children
    pub fn with_child(self, child: Self) -> Self {
        let mut tree = self;
        tree.head.as_mut().unwrap().children.push(child.head.unwrap());
        tree.head.as_mut().unwrap().children.sort();
        tree
    }

    pub fn pov_from(self: &mut Tree<T>, from: &T) -> bool {
        if self.head.as_ref().unwrap().label == *from {
            return true;
        }
        if self.head.as_ref().unwrap().children.is_empty() {
            return false;
        }
        let mut nodes = vec![];
        if self.head.as_mut().unwrap().take(from, &mut nodes) {
            let mut head = self.head.take().unwrap();
            for mut node in nodes.into_iter().rev() {
                node.children.push(head);
                node.children.sort();
                head = node;
            }
            self.head = Some(head);
            return true;
        }
        false
    }

    pub fn path_between<'a>(&'a mut self, from: &T, to: &T) -> Option<Vec<&'a T>> {
        let mut path = Vec::new();
        if !self.pov_from(from) {
            return None;
        }
        if self.head.as_mut().unwrap().find_path(to, &mut path) {
            return Some(path.into_iter().rev().collect());
        }
        None
    }
}
