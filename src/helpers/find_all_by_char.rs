pub fn find_all_by_char(input: &str, pattern: char) -> Vec<usize> {
    let mut occurrences = Vec::new();
    let mut start = 0;

    let _continue = false;

    while let Some(pos) = input[start..].find(pattern) {
        if &input[start..][pos + 1..pos + 2] == "$" {
            start += 1;
        } else {
            let absolute_pos = start + pos;
            occurrences.push(absolute_pos);
            start = absolute_pos + 1;
        }
    }

    occurrences
}
