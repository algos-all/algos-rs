pub fn unique<T>(xs: &mut Vec<T>)
where
    T: Ord,
{
    xs.sort();
    xs.dedup();
}

#[cfg(test)]
mod tests {
    use crate::unique::unique;

    #[test]
    fn unique_noop() {
        let mut xs = vec![1, 2, 3];
        let ys = vec![1, 2, 3];

        unique(&mut xs);

        assert_eq!(xs, ys);
    }

    #[test]
    fn unique_simple_0() {
        let mut xs = vec![0, 0, 0, 0];
        let ys = vec![0];

        unique(&mut xs);

        assert_eq!(xs, ys);
    }

    #[test]
    fn unique_simple_1() {
        let mut xs = vec![0, 1, 2, 0, 2, 1];
        let ys = vec![0, 1, 2];

        unique(&mut xs);

        assert_eq!(xs, ys);
    }
}
