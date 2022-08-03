use rand::seq::SliceRandom;

pub trait SortAlgorithm {
    fn sort<const N: usize>(xs: [i32; N]) -> [i32; N];
}

struct QuickSort;

impl QuickSort {
    fn sort<const N: usize>(
        mut xs: [i32; N],
        lft: usize,
        rgt: usize,
    ) -> [i32; N] {
        if (lft == rgt) || (lft + 1 == rgt) {
            return xs;
        }

        let mut i = lft;
        let mut j = rgt;
        let mut rng = rand::thread_rng();
        while i < j {
            let pivot = *xs[i..j].choose(&mut rng).unwrap();

            while i < j && xs[i] <= pivot {
                i += 1;
            }
            while i < j && xs[j - 1] >= pivot {
                j -= 1;
            }

            if i < j {
                (xs[i], xs[j - 1]) = (xs[j - 1], xs[i])
            }

            Self::sort(xs, lft, i);
            Self::sort(xs, j, rgt);
        }

        xs
    }
}

impl SortAlgorithm for QuickSort {
    fn sort<const N: usize>(xs: [i32; N]) -> [i32; N] {
        Self::sort(xs, 0, xs.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_0() {
        let xs = [];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, []);
    }

    #[test]
    fn test_quick_sort_1() {
        let xs = [42];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, [42]);
    }

    #[test]
    fn test_quick_sort_2_0() {
        let xs = [0, 1];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, [0, 1]);
    }

    #[test]
    fn test_quick_sort_2_1() {
        let xs = [1, 0];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, [0, 1]);
    }

    #[test]
    fn test_quick_sort_4_0() {
        let xs = [1, 2, 3, 4];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, [1, 2, 3, 4]);
    }

    #[test]
    fn test_quick_sort_4_1() {
        let xs = [1, 3, 2, 4];
        let ys = <QuickSort as SortAlgorithm>::sort(xs);

        assert_eq!(ys, [1, 2, 3, 4]);
    }
}
