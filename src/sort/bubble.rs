
pub fn sort<T: Ord>(arr: &mut [T]) {
    let mut n = arr.len();

    loop {
        let mut m = 0;
        for i in range(1, n) {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                m = i;
            }
        }
        n = m;

        if n == 0 {
            break;
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
