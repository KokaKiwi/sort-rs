
pub fn sort<T: Ord>(arr: &mut [T]) {
    for i in range(1, arr.len()) {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::sort;

    use test::BenchHarness;
    use rand::{task_rng, Rng};

    #[test]
    fn test_simple() {
        let mut arr = ~[1, 5, 4, 3, 2, -2, 6];
        sort(arr);

        assert_eq!(arr, ~[-2, 1, 2, 3, 4, 5, 6]);
    }

    #[bench]
    fn bench_128(b: &mut BenchHarness) {
        let mut rng = task_rng();
        let arr: ~[uint] = rng.gen_vec(128);

        b.iter(|| {
            let mut arr = arr.clone();
            sort(arr);
        });
    }

    #[bench]
    fn bench_1024(b: &mut BenchHarness) {
        let mut rng = task_rng();
        let arr: ~[uint] = rng.gen_vec(1024);

        b.iter(|| {
            let mut arr = arr.clone();
            sort(arr);
        });
    }
}

