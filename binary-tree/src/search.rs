use crate::{new_link, BinaryTree, Link, Node};

impl<T> Node<T>
where
    T: Ord,
{
    fn find(&self, value: T) -> bool {
        if self.value == value {
            true
        } else if self.value > value {
            walk_find(&self.left, value)
        } else {
            walk_find(&self.right, value)
        }
    }

    fn insert(&mut self, value: T) {
        if self.value >= value {
            match &mut self.left {
                None => self.left = new_link(value),
                Some(node) => node.insert(value),
            }
        } else {
            match &mut self.right {
                None => self.right = new_link(value),
                Some(node) => node.insert(value),
            }
        }
    }
}

fn walk_find<T>(link: &Link<T>, value: T) -> bool
where
    T: Ord,
{
    match link {
        None => false,
        Some(node) => node.find(value),
    }
}

#[allow(dead_code)]
impl<T> BinaryTree<T>
where
    T: Ord,
{
    fn find(&self, value: T) -> bool {
        walk_find(&self.root, value)
    }

    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(Node {
                    value,
                    left: None,
                    right: None,
                }))
            }
            Some(node) => node.insert(value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{test::create_tree, BinaryTree};

    #[test]
    fn find() {
        let t = create_tree();
        assert!(t.find(10));
        assert!(t.find(0));
        assert!(t.find(-5));
        assert!(t.find(5));
        assert!(t.find(20));
        assert!(t.find(15));
        assert!(t.find(14));
        assert!(t.find(16));
        assert!(t.find(25));
        assert!(!t.find(30));
    }

    #[test]
    fn insert() {
        let w = create_tree();
        let mut g = BinaryTree { root: None };
        g.insert(10);
        g.insert(0);
        g.insert(20);
        g.insert(15);
        g.insert(14);
        g.insert(25);
        g.insert(-5);
        g.insert(16);
        g.insert(5);
        assert_eq!(g, w);
    }
}
