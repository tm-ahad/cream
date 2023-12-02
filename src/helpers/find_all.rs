use crate::std_err::ErrType::SyntaxError;
use crate::consts::NEW_LINE;
use crate::std_err::StdErr;

pub fn find_all(input: &str, pattern: &str, f_name: &str) -> Vec<usize> {
    let mut occurrences = Vec::new();
    let mut start = 0;
    let len = pattern.len();
    let input_len = input.len();

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
                if n == input_len-2 {
                    StdErr::exec(SyntaxError, &format!("'\\n' or 'expected' ({f_name})"));
                }
                n += 1;
            }

            start = n;
        }
    }

    occurrences
}
