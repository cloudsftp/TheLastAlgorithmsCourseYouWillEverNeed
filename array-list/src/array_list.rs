use anyhow::{anyhow, Error};

#[allow(dead_code)]
struct ArrayList<T> {
    length: usize,
    capacity: usize,
    arr: Box<[T]>,
}

const INITIAL_CAP: usize = 5;

#[allow(dead_code)]
impl<T> ArrayList<T>
where
    T: Copy + Default,
{
    fn new() -> Self {
        Self {
            length: 0,
            capacity: INITIAL_CAP,
            arr: vec![T::default(); INITIAL_CAP].into_boxed_slice(),
        }
    }

    fn push(&mut self, val: T) {
        if self.length == self.capacity {
            let new_capacity = self.capacity * 2;
            let mut new_arr = vec![T::default(); new_capacity].into_boxed_slice();
            new_arr[..self.capacity].copy_from_slice(&self.arr);
            self.arr = new_arr;
            self.capacity = new_capacity;
        }

        self.arr[self.length] = val;
        self.length += 1;
    }

    fn at(&self, i: usize) -> Result<T, Error> {
        if i >= self.length {
            return Err(anyhow!("index {i} out of bounds"));
        }

        Ok(self.arr[i])
    }

    fn pop(&mut self) -> Result<T, Error> {
        if self.length == 0 {
            return Err(anyhow!("empty list"));
        }

        self.length -= 1;
        Ok(self.arr[self.length])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = ArrayList::<i32>::new();

        a.push(0);
        a.push(1);
        a.push(2);
        a.push(3);

        assert!(a.at(0).is_ok_and(|i| i == 0));
        assert!(a.at(1).is_ok_and(|i| i == 1));
        assert!(a.at(2).is_ok_and(|i| i == 2));
        assert!(a.at(3).is_ok_and(|i| i == 3));
        assert!(a.at(4).is_err());

        assert!(a.pop().is_ok_and(|i| i == 3));
        assert!(a.pop().is_ok_and(|i| i == 2));
        assert!(a.pop().is_ok_and(|i| i == 1));
        assert!(a.pop().is_ok_and(|i| i == 0));
        assert!(a.pop().is_err());
    }
}
