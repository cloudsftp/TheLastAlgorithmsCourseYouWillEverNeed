#![feature(test)]

mod equality;
mod search;
mod traversal_bredth;
mod traversal_functional;
mod traversal_procedural;

type Link<T> = Option<Box<Node<T>>>;

fn new_link<T>(value: T) -> Link<T> {
    Some(Box::new(Node {
        value,
        left: None,
        right: None,
    }))
}

#[derive(Debug, Eq)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

#[derive(Debug, Eq)]
pub struct BinaryTree<T> {
    root: Link<T>,
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn create_tree() -> BinaryTree<i32> {
        BinaryTree {
            root: Some(Box::new(Node {
                value: 10,
                left: Some(Box::new(Node {
                    value: 0,
                    left: Some(Box::new(Node {
                        value: -5,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 5,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 20,
                    left: Some(Box::new(Node {
                        value: 15,
                        left: Some(Box::new(Node {
                            value: 14,
                            left: None,
                            right: None,
                        })),
                        right: Some(Box::new(Node {
                            value: 16,
                            left: None,
                            right: None,
                        })),
                    })),
                    right: Some(Box::new(Node {
                        value: 25,
                        left: None,
                        right: None,
                    })),
                })),
            })),
        }
    }
}
