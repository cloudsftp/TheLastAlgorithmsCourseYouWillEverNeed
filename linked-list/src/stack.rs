type Link<T> = Option<Box<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

#[allow(dead_code)]
struct Stack<T> {
    head: Link<T>,
}

#[allow(dead_code)]
impl<T> Stack<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|n| n.val.clone())
    }

    fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.val)
    }

    fn push(&mut self, val: T) {
        let node = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }));
        self.head = node;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = Stack::new();

        assert!(s.peek().is_none());
        s.push(0);
        assert!(s.peek().is_some_and(|i| i == 0));
        assert!(s.pop().is_some_and(|i| i == 0));
        assert!(s.peek().is_none());
        assert!(s.pop().is_none());

        s.push(1);
        s.push(2);
        s.push(3);
        assert!(s.peek().is_some_and(|i| i == 3));
        assert!(s.pop().is_some_and(|i| i == 3));
        assert!(s.pop().is_some_and(|i| i == 2));
        assert!(s.pop().is_some_and(|i| i == 1));
        assert!(s.pop().is_none());
    }
}
