use std::ops::Range;

pub fn clone_range<'a>(s: &mut String, rng: Range<usize>) -> &'a str {
    let bytes = s.as_bytes();
    let mut res = String::new();
    let br = &bytes[rng];

    for b in br {
        res.push(*b as char)
    }

    &res
}