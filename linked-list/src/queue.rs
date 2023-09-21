use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[allow(dead_code)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

#[allow(dead_code)]
struct Queue<T> {
    length: usize,
    head: Link<T>,
    tail: Link<T>,
}

#[allow(dead_code)]
impl<T> Queue<T>
where
    T: Copy,
{
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|r| r.borrow().val)
    }

    fn deque(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.borrow().next.clone();

        self.length -= 1;
        if self.length == 0 {
            self.tail = None;
        }

        let res = node.borrow().val;
        Some(res)
    }

    fn enqueue(&mut self, val: T) {
        let node = Some(Rc::new(RefCell::new(Node { val, next: None })));
        if self.length == 0 {
            self.head = node.clone();
            self.tail = node;
        } else {
            self.tail
                .as_ref()
                .expect("this has to be some")
                .borrow_mut()
                .next = node.clone();
            self.tail = node;
        }
        self.length += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut q = Queue::new();

        assert!(q.peek().is_none());
        q.enqueue(0);
        assert!(q.peek().is_some_and(|i| i == 0));
        assert!(q.deque().is_some_and(|i| i == 0));
        assert!(q.peek().is_none());
        assert!(q.deque().is_none());

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert!(q.peek().is_some_and(|i| i == 1));
        assert!(q.deque().is_some_and(|i| i == 1));
        assert!(q.deque().is_some_and(|i| i == 2));
        assert!(q.deque().is_some_and(|i| i == 3));
        assert!(q.deque().is_none());
    }
}
