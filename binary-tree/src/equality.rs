use crate::{BinaryTree, Node};

#[allow(dead_code)]
impl<T> PartialEq for Node<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}

#[allow(dead_code)]
impl<T> PartialEq for BinaryTree<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_eq() {
        let t1 = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        let t2 = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        assert_eq!(t1, t2);
    }

    #[test]
    fn medium_eq() {
        let t1 = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 2,
                    left: None,
                    right: None,
                })),
            })),
        };
        let t2 = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 2,
                    left: None,
                    right: None,
                })),
            })),
        };
        let t3 = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: Some(Box::new(Node {
                    value: 1,
                    left: Some(Box::new(Node {
                        value: 2,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
        };
        assert_eq!(t1, t2);
        assert_ne!(t1, t3);
    }
}
