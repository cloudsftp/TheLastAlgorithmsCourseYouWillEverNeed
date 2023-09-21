use std::fmt::Display;

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
impl<T> List<T>
where
    T: Copy,
{
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    fn pop(&mut self) -> Result<T, Error> {
        let node = self.head.take().ok_or(anyhow!("empty list"))?;
        self.head = node.next;
        Ok(node.val)
    }

    fn get_at(&mut self, i: usize) -> Result<T, Error> {
        let mut node = self.head.as_ref().ok_or(anyhow!("empty list"))?;
        for _ in 0..i {
            node = node
                .next
                .as_ref()
                .ok_or(anyhow!("index {i} out of bounds"))?;
        }

        Ok(node.val)
    }

    fn delete_at(&mut self, i: usize) -> Result<T, Error> {
        if i == 0 {
            return self.pop();
        }

        let mut last = self.head.as_mut().ok_or(anyhow!("empty list"))?;
        for _ in 1..i {
            last = last
                .next
                .as_mut()
                .ok_or(anyhow!("index {i} is out of bounds"))?;
        }

        let curr = last
            .next
            .take()
            .ok_or(anyhow!("index {i} is out of bounds"))?;
        last.next = curr.next;

        Ok(curr.val)
    }

    fn insert_at(&mut self, i: usize, val: T) -> Result<(), Error> {
        if i == 0 {
            self.push(val);
            return Ok(());
        }

        let mut last = self.head.as_mut().ok_or(anyhow!("empty list"))?;
        for _ in 1..i {
            last = last
                .next
                .as_mut()
                .ok_or(anyhow!("index {i} is out of bounds"))?;
        }

        let next = last.next.take();
        let new = Some(Box::new(Node { val, next }));
        last.next = new;

        Ok(())
    }
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let mut link = self.head.as_ref();
        let mut i = 0;
        while let Some(node) = link {
            if i > 0 {
                f.write_str(", ")?;
            }
            i += 1;
            f.write_fmt(format_args!("{}", node.val))?;
            link = node.next.as_ref();
        }
        f.write_str("]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_at() {
        let mut l: List<i32> = List::new();
        l.push(2);
        l.push(1);
        l.push(0);
        assert!(l.insert_at(1, -1).is_ok());
        assert!(l.get_at(1).is_ok_and(|i| i == -1));
        assert!(l.get_at(0).is_ok_and(|i| i == 0));
        assert!(l.get_at(2).is_ok_and(|i| i == 1));
        assert!(l.get_at(3).is_ok_and(|i| i == 2));
    }

    #[test]
    fn delete_at() {
        let mut l: List<i32> = List::new();
        l.push(2);
        l.push(1);
        l.push(0);
        assert!(l.delete_at(1).is_ok_and(|i| i == 1));
        assert!(l.delete_at(3).is_err());
        assert!(l.get_at(0).is_ok_and(|i| i == 0));
        assert!(l.get_at(1).is_ok_and(|i| i == 2));
    }

    #[test]
    fn get_at() {
        let mut l: List<i32> = List::new();
        l.push(0);
        l.push(1);
        l.push(2);
        assert!(l.get_at(1).is_ok_and(|i| i == 1));
        assert!(l.get_at(3).is_err());
    }

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
