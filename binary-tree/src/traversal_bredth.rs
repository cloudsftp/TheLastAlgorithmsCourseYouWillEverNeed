use std::collections::VecDeque;

use crate::BinaryTree;

#[allow(dead_code)]
impl<T> BinaryTree<T>
where
    T: Copy + Eq,
{
    fn breadth_first(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut q = VecDeque::from([&self.root]);

        while !q.is_empty() {
            let curr = q.pop_front().expect("Only popping from non-empty queue");
            if let Some(node) = curr {
                q.push_back(&node.left);
                q.push_back(&node.right);
                res.push(node.value);
            }
        }

        res
    }

    fn breadth_first_search(&self, n: T) -> bool {
        let mut q = VecDeque::from([&self.root]);

        while !q.is_empty() {
            let curr = q.pop_front().expect("Only popping from non-empty queue");
            if let Some(node) = curr {
                if node.value == n {
                    return true;
                }
                q.push_back(&node.left);
                q.push_back(&node.right);
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use crate::test::create_tree;

    #[test]
    fn breadth_first() {
        let t = create_tree();
        assert_eq!(t.breadth_first(), [10, 0, 20, -5, 5, 15, 25, 14, 16]);
    }

    #[test]
    fn breadth_first_search() {
        let t = create_tree();
        assert!(t.breadth_first_search(10));
        assert!(t.breadth_first_search(0));
        assert!(t.breadth_first_search(20));
        assert!(t.breadth_first_search(-5));
        assert!(t.breadth_first_search(5));
        assert!(t.breadth_first_search(15));
        assert!(t.breadth_first_search(25));
        assert!(t.breadth_first_search(14));
        assert!(t.breadth_first_search(16));
        assert!(!t.breadth_first_search(-1));
    }
}
