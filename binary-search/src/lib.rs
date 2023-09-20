#![allow(dead_code)]

fn internal_search<T>(arr: &Vec<T>, lo: usize, hi: usize, v: T) -> Option<usize>
where
    T: Ord,
{
    if hi == lo {
        return None;
    }

    let m = lo + (hi - lo) / 2;
    if arr[m] == v {
        Some(m)
    } else if arr[m] < v {
        internal_search(arr, m + 1, hi, v)
    } else {
        internal_search(arr, lo, m, v)
    }
}

fn search_rec<T>(arr: Vec<T>, v: T) -> Option<usize>
where
    T: Ord,
{
    internal_search(&arr, 0, arr.len(), v)
}

fn search<T>(arr: Vec<T>, v: T) -> Option<usize>
where
    T: Ord,
{
    let mut lo = 0;
    let mut hi = arr.len();

    while lo < hi {
        let m = lo + (hi - lo) / 2;

        if arr[m] == v {
            return Some(m);
        } else if arr[m] < v {
            lo = m + 1;
        } else {
            hi = m;
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_middle() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search(arr, 2), Some(2));
    }

    #[test]
    fn test_left_middle() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search(arr, 1), Some(1));
    }

    #[test]
    fn test_not_found() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search(arr, 5), None);
    }

    #[test]
    fn test_even() {
        let arr = vec![0, 1, 2, 3];
        assert_eq!(search(arr, 2), Some(2));
    }

    #[test]
    fn test_rec_middle() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search_rec(arr, 2), Some(2));
    }

    #[test]
    fn test_rec_left_middle() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search_rec(arr, 1), Some(1));
    }

    #[test]
    fn test_rec_not_found() {
        let arr = vec![0, 1, 2, 3, 4];
        assert_eq!(search_rec(arr, 5), None);
    }

    #[test]
    fn test_rec_even() {
        let arr = vec![0, 1, 2, 3];
        assert_eq!(search_rec(arr, 2), Some(2));
    }
}
