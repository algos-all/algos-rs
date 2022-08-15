use rand::Rng;

pub trait SortAlgorithm {
    fn sort(xs: &mut [i32]);
}

pub struct QuickSort;

impl QuickSort {
    fn sort(xs: &mut [i32], lft: usize, rgt: usize) {
        if lft >= rgt {
            return;
        }

        let (mut i, mut j) = (lft, rgt);
        let (mut overflow_add, mut overflow_sub) = (false, false);
        // TODO: initialize random number generator less often:
        let mut rng = rand::thread_rng();
        let pivot = xs[rng.gen_range(lft..rgt)];
        // let pivot = xs[i + (j - i) / 2];

        while !(overflow_add || overflow_sub) && i < j {
            while !overflow_add && xs[i] < pivot {
                (i, overflow_add) = i.overflowing_add(1);
            }
            while !overflow_sub && xs[j] > pivot {
                (j, overflow_sub) = j.overflowing_sub(1);
            }

            if !(overflow_add || overflow_sub) && i <= j {
                xs.swap(i, j);
                (i, overflow_add) = i.overflowing_add(1);
                (j, overflow_sub) = j.overflowing_sub(1);
            }
        }

        if !overflow_sub {
            Self::sort(xs, lft, j);
        }
        if !overflow_add {
            Self::sort(xs, i, rgt);
        }
    }
}

impl SortAlgorithm for QuickSort {
    fn sort(xs: &mut [i32]) {
        if xs.is_empty() {
            return;
        }

        Self::sort(xs, 0, xs.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_quick_sort_0() {
        let mut xs = [];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, []);
    }

    #[test]
    fn test_quick_sort_1() {
        let mut xs = vec![42];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [42]);
    }

    #[test]
    fn test_quick_sort_2_0() {
        let mut xs = vec![0, 1];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [0, 1]);
    }

    #[test]
    fn test_quick_sort_2_1() {
        let mut xs = vec![1, 0];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [0, 1]);
    }

    #[test]
    fn test_quick_sort_2_2() {
        let mut xs = vec![2, 2];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [2, 2]);
    }

    #[test]
    fn test_quick_sort_4_0() {
        let mut xs = vec![1, 2, 3, 4];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_4_1() {
        let mut xs = vec![1, 3, 2, 4];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_4_2() {
        let mut xs = vec![1, 4, 2, 3];
        <QuickSort as SortAlgorithm>::sort(&mut xs);

        assert_eq!(xs, [1, 2, 3, 4]);
    }

    #[quickcheck]
    fn test_random_input(mut xs: Vec<i32>) -> bool {
        let mut ys = xs.clone();
        ys.sort_unstable();
        <QuickSort as SortAlgorithm>::sort(&mut xs);
        ys == xs
    }
}
