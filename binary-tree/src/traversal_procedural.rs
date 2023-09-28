use crate::{BinaryTree, Link, Node};

fn walk_link_proc<T>(link: &Link<T>, res: &mut Vec<T>, f: fn(&Box<Node<T>>, &mut Vec<T>) -> ()) {
    match link {
        None => (),
        Some(node) => f(node, res),
    }
}

fn walk_in_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_proc(link, res, |node, res| {
        walk_in_order(&node.left, res);
        res.push(node.value);
        walk_in_order(&node.right, res);
    });
}

fn walk_pre_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_proc(link, res, |node, res| {
        res.push(node.value);
        walk_pre_order(&node.left, res);
        walk_pre_order(&node.right, res);
    });
}

fn walk_post_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_proc(link, res, |node, res| {
        walk_post_order(&node.left, res);
        walk_post_order(&node.right, res);
        res.push(node.value);
    });
}

#[allow(dead_code)]
impl<T> BinaryTree<T>
where
    T: Copy,
{
    fn in_order_proc(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_in_order(&self.root, &mut res);
        res
    }

    fn pre_order_proc(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_pre_order(&self.root, &mut res);
        res
    }

    fn post_order_proc(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_post_order(&self.root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::test::create_tree;

    #[test]
    fn in_order_proc() {
        let t = create_tree();
        assert_eq!(t.in_order_proc(), [-5, 0, 5, 10, 14, 15, 16, 20, 25]);
    }

    #[test]
    fn pre_order_proc() {
        let t = create_tree();
        assert_eq!(t.pre_order_proc(), [10, 0, -5, 5, 20, 15, 14, 16, 25]);
    }

    #[test]
    fn post_order_proc() {
        let t = create_tree();
        assert_eq!(t.post_order_proc(), [-5, 5, 0, 14, 16, 15, 25, 20, 10]);
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_in_order_proc(b: &mut Bencher) {
        let t = create_tree();
        b.iter(|| t.in_order_proc())
    }
}
