/// PriorityQueue implements a min-heap.
/// So the first element is always the smallest element in the queue.
#[allow(dead_code)]
struct PriorityQueue<T> {
    a: Vec<T>,
}

#[allow(dead_code)]
impl<T> PriorityQueue<T>
where
    T: Ord + Copy,
{
    fn new() -> Self {
        Self { a: Vec::new() }
    }

    fn heapify_down(&mut self, i: usize) {
        let l = 2 * i + 1;
        if l >= self.a.len() {
            return;
        }
        let r = l + 1;
        let c = if r >= self.a.len() || self.a[l] < self.a[r] {
            l
        } else {
            r
        };
        if self.a[i] > self.a[c] {
            self.a.swap(i, c);
            self.heapify_down(c);
        }
    }

    fn heapify_up(&mut self, i: usize) {
        if i < 1 {
            return;
        }

        let p = (i - 1) / 2;
        if self.a[p] > self.a[i] {
            self.a.swap(p, i);
            self.heapify_up(p);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.a.len() > 0 {
            let res = self.a[0];
            let last = self.a.pop().expect("length of array is bigger than one");
            if self.a.len() > 0 {
                self.a[0] = last;
                self.heapify_down(0);
            }
            Some(res)
        } else {
            None
        }
    }

    fn push(&mut self, value: T) {
        self.a.push(value);
        self.heapify_up(self.a.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut q = PriorityQueue::new();
        let mut elements = [300, 20, 40, 10, 1, 0, -100, 30, 30, 30, 30, 420];
        for v in elements {
            q.push(v);
        }

        elements.sort();
        for v in elements {
            assert_eq!(q.pop().unwrap(), v);
        }
        assert!(q.pop().is_none());
    }
}
