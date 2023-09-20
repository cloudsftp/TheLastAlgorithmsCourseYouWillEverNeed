#[allow(dead_code)]
fn search(breaks: Vec<bool>) -> Option<usize> {
    if breaks.len() == 0 || breaks[0] {
        return None;
    }

    let step = (breaks.len() as f64).sqrt() as usize;
    let mut i = 0usize;

    while i + step < breaks.len() && !breaks[i + step] {
        i += step
    }

    while i < breaks.len() && !breaks[i] {
        i += 1;
    }

    Some(i)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_breaks() {
        let breaks = vec![false, false, false, false, false, true, true, true, true];
        assert_eq!(search(breaks), Some(5));
    }

    #[test]
    fn test_not() {
        let breaks = vec![false, false, false, false, false];
        assert_eq!(search(breaks), Some(5));
    }

    #[test]
    fn test_empty() {
        let breaks = vec![];
        assert_eq!(search(breaks), None);
    }

    #[test]
    fn test_breaks_always() {
        let breaks = vec![true];
        assert_eq!(search(breaks), None);
    }
}
