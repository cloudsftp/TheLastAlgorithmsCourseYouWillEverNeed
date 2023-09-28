use crate::{BinaryTree, Link, Node};

fn walk_link<T>(link: &Link<T>, f: fn(&Box<Node<T>>) -> Vec<T>) -> Vec<T> {
    match link {
        None => Vec::new(),
        Some(node) => f(node),
    }
}

fn inorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    [
        walk_link(&node.left, inorder_step),
        vec![node.value],
        walk_link(&node.right, inorder_step),
    ]
    .concat()
}

fn preorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    [
        vec![node.value],
        walk_link(&node.left, preorder_step),
        walk_link(&node.right, preorder_step),
    ]
    .concat()
}

fn postorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    [
        walk_link(&node.left, postorder_step),
        walk_link(&node.right, postorder_step),
        vec![node.value],
    ]
    .concat()
}

#[allow(dead_code)]
impl<T> BinaryTree<T>
where
    T: Copy,
{
    fn in_order(&self) -> Vec<T> {
        walk_link(&self.root, inorder_step)
    }

    fn pre_order(&self) -> Vec<T> {
        walk_link(&self.root, preorder_step)
    }

    fn post_order(&self) -> Vec<T> {
        walk_link(&self.root, postorder_step)
    }
}

#[cfg(test)]
mod test {
    use crate::test::create_tree;

    #[test]
    fn in_order() {
        let t = create_tree();
        assert_eq!(t.in_order(), [-5, 0, 5, 10, 14, 15, 16, 20, 25]);
    }

    #[test]
    fn pre_order() {
        let t = create_tree();
        assert_eq!(t.pre_order(), [10, 0, -5, 5, 20, 15, 14, 16, 25]);
    }

    #[test]
    fn post_order() {
        let t = create_tree();
        assert_eq!(t.post_order(), [-5, 5, 0, 14, 16, 15, 25, 20, 10]);
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_in_order_func(b: &mut Bencher) {
        let t = create_tree();
        b.iter(|| t.in_order())
    }
}
