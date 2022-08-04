use std::collections::VecDeque;

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }
    fn traverse_level(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();
        let mut q: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        results.push(root.value);
        q.push_back(root);

        let mut height = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    if let Some(ref left) = node.left {
                        results.push(left.value);
                        q.push_back(left);
                    }
                    if let Some(ref right) = node.right {
                        results.push(right.value);
                        q.push_back(right);
                    }
                }
            }
            height += 1;
        }

        println!("height:{}", height);
        results
    }
    fn insert(&mut self, value: i32) {
        /* calling insert_recursive fn
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            }
            Some(n) => {
                Tree::insert_recursive(n, value);
            }
        }*/
        //calling insert_iterative fn
        self.insert_iterative(value);
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if value > node.value {
                let right = &mut node.right;
                match right {
                    None => {
                        *right = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            } else if value < node.value {
                let left = &mut node.left;
                match left {
                    None => {
                        *left = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(node) => {
                    Tree::insert_recursive(node, value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_tree() {
        let mut tree = Tree::new();
        tree.insert(9);
        tree.insert(10);
        tree.insert(8);
        tree.insert(5);
        tree.insert(13);
        tree.insert(7);
        tree.insert(11);
        tree.insert(3);
        assert_eq!(tree.root.is_some(), true);

        println!("{:?}", tree);
    }
    #[test]

    fn test_level_traverse_tree() {
        let mut tree = Tree::new();
        tree.insert(9);
        tree.insert(10);
        tree.insert(8);
        tree.insert(5);
        tree.insert(13);
        tree.insert(7);
        tree.insert(11);
        tree.insert(3);
        assert_eq!(tree.traverse_level(), vec!(9, 8, 10, 5, 13, 3, 7, 11))
    }
}
