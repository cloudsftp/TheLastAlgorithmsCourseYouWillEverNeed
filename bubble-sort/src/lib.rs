#[allow(dead_code)]
fn sort<T>(arr: &mut Vec<T>)
where
    T: Ord,
{
    for n in (1..arr.len()).rev() {
        for i in 0..n {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut arr = vec![1, 3, 2];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn it_works_longer() {
        let mut arr = vec![1, 3, 2, 5, 9, 0, 20, 300, 4, 5, 6, 2, 1];
        sort(&mut arr);
        assert_eq!(arr, [0, 1, 1, 2, 2, 3, 4, 5, 5, 6, 9, 20, 300]);
    }
}
