mod traversal;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

struct BinaryTree<T> {
    root: Link<T>,
}
