pub fn collect_arr<const N: usize, T: Default + Copy>(mut iter: impl Iterator<Item = T>) -> [T; N] {
    let mut res = [T::default(); N];

    for i in 0..N {
        res[i] = iter.next().unwrap();
    }

    res
}
