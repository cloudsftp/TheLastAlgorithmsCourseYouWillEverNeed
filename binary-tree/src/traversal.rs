use crate::{BinaryTree, Link, Node};

// Without out parameter

fn walk_link<T>(link: &Link<T>, f: fn(&Box<Node<T>>) -> Vec<T>) -> Vec<T> {
    link.as_ref().map(f).unwrap_or(Vec::new())
}

fn inorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    let mut res = walk_link(&node.left, inorder_step);
    res.push(node.value);
    res.append(&mut walk_link(&node.right, inorder_step));
    res
}

fn preorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    let mut res = vec![node.value];
    res.append(&mut walk_link(&node.left, preorder_step));
    res.append(&mut walk_link(&node.right, preorder_step));
    res
}

fn postorder_step<T>(node: &Box<Node<T>>) -> Vec<T>
where
    T: Copy,
{
    let mut res = walk_link(&node.left, postorder_step);
    res.append(&mut walk_link(&node.right, postorder_step));
    res.push(node.value);
    res
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

// With out parameter

fn walk_link_out_param<T>(
    link: &Link<T>,
    res: &mut Vec<T>,
    f: fn(&Box<Node<T>>, &mut Vec<T>) -> (),
) {
    match link {
        None => (),
        Some(node) => f(node, res),
    }
}

fn walk_in_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_out_param(link, res, |node, res| {
        walk_in_order(&node.left, res);
        res.push(node.value);
        walk_in_order(&node.right, res);
    });
}

fn walk_pre_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_out_param(link, res, |node, res| {
        res.push(node.value);
        walk_pre_order(&node.left, res);
        walk_pre_order(&node.right, res);
    });
}

fn walk_post_order<T>(link: &Link<T>, res: &mut Vec<T>)
where
    T: Copy,
{
    walk_link_out_param(link, res, |node, res| {
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
    fn in_order_out_param(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_in_order(&self.root, &mut res);
        res
    }

    fn pre_order_out_param(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_pre_order(&self.root, &mut res);
        res
    }

    fn post_order_out_param(&self) -> Vec<T> {
        let mut res = Vec::new();
        walk_post_order(&self.root, &mut res);
        res
    }
}

// Tests
////////

#[cfg(test)]
mod tests {
    use super::*;

    fn create_tree() -> BinaryTree<i32> {
        BinaryTree {
            root: Some(Box::new(Node {
                value: 10,
                left: Some(Box::new(Node {
                    value: 0,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 5,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 20,
                    left: None,
                    right: None,
                })),
            })),
        }
    }

    #[test]
    fn in_order() {
        let t = create_tree();
        assert_eq!(t.in_order(), [0, 5, 10, 20]);
    }

    #[test]
    fn pre_order() {
        let t = create_tree();
        assert_eq!(t.pre_order(), [10, 0, 5, 20]);
    }

    #[test]
    fn post_order() {
        let t = create_tree();
        assert_eq!(t.post_order(), [5, 0, 20, 10]);
    }

    #[test]
    fn in_order_out_param() {
        let t = create_tree();
        assert_eq!(t.in_order_out_param(), [0, 5, 10, 20]);
    }

    #[test]
    fn pre_order_out_param() {
        let t = create_tree();
        assert_eq!(t.pre_order_out_param(), [10, 0, 5, 20]);
    }

    #[test]
    fn post_order_out_param() {
        let t = create_tree();
        assert_eq!(t.post_order_out_param(), [5, 0, 20, 10]);
    }
}
