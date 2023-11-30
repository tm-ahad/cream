use crate::consts::NEW_LINE;

pub fn find_all(input: &str, pattern: &str) -> Vec<usize> {
    let mut occurrences = Vec::new();
    let mut start = 0;
    let len = pattern.len();

    let _continue = false;

    while let Some(pos) = input[start..].find(pattern) {
        if &input[start..][pos + 1..pos + pattern.len() + 1] == "$" {
            start += 1;
        } else {
            let absolute_pos = start + pos;
            occurrences.push(absolute_pos);

            let mut n = absolute_pos + len;

            while !(
                &input[n..n+1] == NEW_LINE ||
                &input[n..n+1] == ";"
            ) {
                n += 1;
            }

            start = n;
        }
    }

    occurrences
}
