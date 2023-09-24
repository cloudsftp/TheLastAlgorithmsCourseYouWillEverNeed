#[allow(dead_code)]
fn partition<T>(a: &mut [T]) -> usize
where
    T: Ord,
{
    let p = a.len() - 1;
    let mut left = 0;

    a.swap(a.len() / 2, p);
    for right in 0..p {
        if a[right] <= a[p] {
            a.swap(left, right);
            left += 1;
        }
    }
    a.swap(left, p);

    left
}

#[allow(dead_code)]
fn quick_sort<T>(a: &mut [T])
where
    T: Ord,
{
    if a.len() <= 1 {
        return;
    }

    let mid = partition(a);

    if mid > 0 {
        quick_sort(&mut a[..mid]);
    }
    if mid + 1 < a.len() {
        quick_sort(&mut a[mid + 1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn is_sorted<T>(a: &[T]) -> bool
    where
        T: Ord,
    {
        for i in 0..a.len() - 1 {
            if a[i] > a[i + 1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn it_works() {
        let mut a = [1, 5, 7, 3, 9, 6, 7, 5, 4, 5, 6, 7, 8];
        quick_sort(&mut a);
        println!("{a:?}");
        assert!(is_sorted(&a));
    }

    #[test]
    fn random() {
        let size = 1000;
        let mut rng = rand::thread_rng();

        let mut a = Vec::<i32>::with_capacity(size);
        for _ in 0..size {
            a.push(rng.gen());
        }

        quick_sort(&mut a);
        assert!(is_sorted(&a));
    }
}
