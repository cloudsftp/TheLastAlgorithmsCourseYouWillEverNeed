use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use anyhow::{anyhow, Error};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
fn new_link<T>(node: Node<T>) -> Link<T> {
    Some(Rc::new(RefCell::new(node)))
}

struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

struct List<T> {
    len: usize,
    head: Link<T>,
    tail: Link<T>,
}

#[allow(dead_code)]
impl<T> List<T>
where
    T: Copy,
{
    fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    fn get(&self, i: usize) -> Result<T, Error> {
        let mut curr = self.head.clone();

        for _ in 0..i {
            match curr {
                None => return Err(anyhow!("index {i} is out of bounds")),
                Some(link) => {
                    curr = link.borrow().next.clone();
                }
            }
        }

        match curr {
            None => Err(anyhow!("index {i} is out of bounds")),
            Some(link) => {
                let ret = link.borrow().val;
                Ok(ret)
            }
        }
    }

    fn append(&mut self, val: T) {
        let mut node = Node {
            val,
            next: None,
            prev: None,
        };

        if self.len == 0 {
            let node = new_link(node);
            self.head = node.clone();
            self.tail = node;
        } else {
            let tail = self.tail.take();
            node.prev = tail.clone();
            if let Some(tail) = tail {
                tail.as_ref().borrow_mut().next = new_link(node);
            }
        }

        self.len += 1;
    }
}

#[cfg(test)]
mod test {
    use super::List;

    fn list_equals<T>(l: &List<T>, v: &[T]) -> bool
    where
        T: Eq + Copy,
    {
        for i in 0..l.len {
            if l.get(i).unwrap() != v[i] {
                return false;
            }
        }
        true
    }

    #[test]
    fn append() {
        let mut l = List::<i32>::new();
        l.append(0);
        l.append(1);

        assert!(list_equals(&l, &[0, 1]))
    }
}
