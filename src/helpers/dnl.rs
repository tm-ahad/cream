pub fn dnl(n: &u32) -> u8 {
    let mut res = 0;
    let mut n = *n;

    while n >= 10 {
        n /= 10;
        res += 1;
    }

    res
}
