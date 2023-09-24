use std::{cell::RefCell, fmt::Display, rc::Rc};

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

    fn walk_to_index(&self, i: usize) -> Result<Rc<RefCell<Node<T>>>, Error> {
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
            Some(link) => Ok(link),
        }
    }

    fn get(&self, i: usize) -> Result<T, Error> {
        let node = self.walk_to_index(i)?;
        let ret = node.borrow().val;
        Ok(ret)
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
            let link = new_link(node);
            if let Some(tail) = tail {
                tail.as_ref().borrow_mut().next = link.clone();
            }
            self.tail = link;
        }

        self.len += 1;
    }

    fn prepend(&mut self, val: T) {
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
            let head = self.head.take();
            node.next = head.clone();
            let link = new_link(node);
            if let Some(head) = head {
                head.as_ref().borrow_mut().prev = link.clone();
            }
            self.head = link;
        }

        self.len += 1;
    }

    fn insert_at(&mut self, i: usize, val: T) -> Result<(), Error> {
        if i < 0 || i > self.len {
            return Err(anyhow!("index {i} is out of bounds"));
        }

        if i == 0 {
            self.prepend(val);
            return Ok(());
        }
        if i == self.len {
            self.append(val);
            return Ok(());
        }

        let next = self.walk_to_index(i)?;
        let prev = next
            .as_ref()
            .borrow_mut()
            .prev
            .take()
            .ok_or(anyhow!("internal problem"))?
            .clone();

        let node = Some(Rc::new(RefCell::new(Node {
            val,
            next: Some(next.clone()),
            prev: Some(prev.clone()),
        })));

        prev.borrow_mut().next = node.clone();
        next.borrow_mut().prev = node;
        Ok(())
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.val))?;
        Ok(())
    }
}

impl<T> Display for List<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr = self.head.clone();
        f.write_str("[")?;

        let mut i = 0;
        loop {
            match curr {
                None => break,
                Some(node) => {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    i += 1;

                    let node = node.borrow();
                    f.write_fmt(format_args!("{}", node))?;
                    curr = node.next.clone();
                }
            }
        }
        f.write_str("]")?;

        Ok(())
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

        assert!(list_equals(&l, &[0, 1]));
    }

    #[test]
    fn prepend() {
        let mut l = List::<i32>::new();
        l.prepend(0);
        l.prepend(1);

        assert!(list_equals(&l, &[1, 0]))
    }

    #[test]
    fn insert() {
        let mut l = List::<i32>::new();
        l.append(0);
        l.append(1);
        l.append(2);
        l.append(3);
        l.append(4);

        l.insert_at(2, 10).unwrap();

        println!("{l}");
        assert!(list_equals(&l, &[0, 1, 10, 2, 3, 4]))
    }
}
