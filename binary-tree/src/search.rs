use std::mem;

use crate::{new_link, BinaryTree, Link, Node};

// Could be impl if I had a proper Link<T> struct :/
fn walk_insert<T>(link: &mut Link<T>, value: T)
where
    T: Ord,
{
    match link {
        None => {
            let _ = mem::replace(link, new_link(value));
        }
        Some(node) => {
            if node.value < value {
                walk_insert(&mut node.right, value)
            } else {
                walk_insert(&mut node.left, value)
            }
        }
    }
}

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
        walk_insert(&mut self.root, value)
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
