pub fn dnl(n: &u32) -> usize {
    let mut res = 1;
    let mut n = *n;

    while n >= 10 {
        n /= 10;
        res += 1;
    }

    res
}
