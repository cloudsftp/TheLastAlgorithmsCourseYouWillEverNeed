use anyhow::{anyhow, Error};

#[allow(dead_code)]
struct RingBuffer<T> {
    start: usize, // including
    end: usize,   // excluding
    capacity: usize,
    arr: Box<[T]>,
}

const INITIAL_CAP: usize = 5;

#[allow(dead_code)]
impl<T> RingBuffer<T>
where
    T: Copy + Default,
{
    fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            capacity: INITIAL_CAP,
            arr: vec![T::default(); INITIAL_CAP].into_boxed_slice(),
        }
    }

    fn push(&mut self, val: T) {
        if self.start == 0 && self.end == self.capacity
            || self.start > 0 && self.end == self.start - 1
        {
            let new_capacity = self.capacity * 2;
            let mut new_array = vec![T::default(); new_capacity].into_boxed_slice();
            if self.start <= self.end {
                new_array[..self.end - self.start].copy_from_slice(&self.arr[self.start..])
            } else {
                let num_elem_right = self.capacity - self.start;
                new_array[..num_elem_right].copy_from_slice(&self.arr[self.start..]);
                new_array[num_elem_right..num_elem_right + self.end].copy_from_slice(&self.arr);
                self.start = 0;
                self.end += num_elem_right;
            }
            self.arr = new_array;
            self.capacity = new_capacity;
        }

        self.arr[self.end] = val;
        self.end = self.end + 1 % self.capacity;
    }

    fn at(&self, i: usize) -> Result<T, Error> {
        if self.start < self.end && i >= self.end - self.start
            || i >= self.capacity - self.start + self.end
        {
            return Err(anyhow!("index {i} is out of bounds"));
        }

        Ok(self.arr[self.start + i % self.capacity])
    }

    fn pop(&mut self) -> Result<T, Error> {
        if self.end == self.start {
            return Err(anyhow!("buffer empty"));
        }

        if self.end > 0 {
            self.end -= 1;
        } else {
            self.end = self.capacity;
        }
        Ok(self.arr[self.end])
    }

    //deque
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_as_array_list() {
        let mut r = RingBuffer::<i32>::new();

        r.push(0);
        r.push(1);
        r.push(2);
        r.push(3);

        assert!(r.at(0).is_ok_and(|i| i == 0));
        assert!(r.at(1).is_ok_and(|i| i == 1));
        assert!(r.at(2).is_ok_and(|i| i == 2));
        assert!(r.at(3).is_ok_and(|i| i == 3));
        assert!(r.at(4).is_err());

        assert!(r.pop().is_ok_and(|i| i == 3));
        assert!(r.pop().is_ok_and(|i| i == 2));
        assert!(r.pop().is_ok_and(|i| i == 1));
        assert!(r.pop().is_ok_and(|i| i == 0));
        assert!(r.pop().is_err());
    }
}
