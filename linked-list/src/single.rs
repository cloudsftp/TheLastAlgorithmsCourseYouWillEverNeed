use anyhow::{anyhow, Error};

type Link<T> = Option<Box<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

#[allow(dead_code)]
struct List<T> {
    head: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, v: T) {
        let node = Box::new(Node {
            val: v,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Result<T, Error> {
        let node = self.head.take().ok_or(anyhow!("empty list"))?;
        self.head = node.next;
        Ok(node.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_works() {
        let mut l: List<i32> = List::new();
        l.push(0);
        l.push(1);
        l.push(2);
        assert!(l.pop().is_ok_and(|i| i == 2));
        assert!(l.pop().is_ok_and(|i| i == 1));
        assert!(l.pop().is_ok_and(|i| i == 0));
        assert!(l.pop().is_err());
    }
}
