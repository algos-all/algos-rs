pub mod binary_search_tree;
pub mod unique;

pub fn lower_bound<T>(xs: &Vec<T>, x: &T) -> usize
where
    T: Ord,
{
    let mut lft = 0 as usize;
    let mut rgt = xs.len();

    while lft < rgt {
        let mid = lft + (rgt - lft) / 2;

        if *x <= xs[mid] {
            rgt = mid;
        } else {
            lft = mid + 1;
        }
    }

    rgt
}

#[cfg(test)]
mod tests {
    use crate::binary_search_tree as bst;
    use crate::lower_bound;
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

    #[test]
    fn test_lower_bound_empty_vector() {
        assert_eq!(lower_bound(&Vec::new(), &0), 0);
        assert_eq!(lower_bound(&Vec::new(), &42), 0);
        assert_eq!(lower_bound(&Vec::new(), &-1), 0);
    }

    #[test]
    fn test_lower_bound_one_element() {
        let xs = vec![42];

        assert_eq!(lower_bound(&xs, &0), 0);
        assert_eq!(lower_bound(&xs, &42), 0);
        assert_eq!(lower_bound(&xs, &142), 1);
    }

    #[test]
    fn test_lower_bound_sequence() {
        let xs = vec![1, 2, 2, 3, 3, 3];

        assert_eq!(lower_bound(&xs, &0), 0);
        assert_eq!(lower_bound(&xs, &1), 0);
        assert_eq!(lower_bound(&xs, &2), 1);
        assert_eq!(lower_bound(&xs, &3), 3);
    }

    #[test]
    fn bst_new() {
        let tree = bst::BinarySearchTree::new(42, 42);

        assert_eq!(tree.root.is_some(), true);
    }
}
