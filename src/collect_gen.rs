
pub fn collect_gen(toks: String, keyword: String, end: &str, found_id: Option<usize>, temp: bool) -> String {
    let binding = toks.split('\n')
        .collect::<Vec<&str>>();

    let spls = binding
        .iter()
        .filter(|a| a.trim() != "")
        .collect::<Vec<&&str>>();

    let len = end.len();
    let spl_len = spls.len();

    for (idx, l) in spls.clone().into_iter().enumerate() {
        if l.replace(' ', "") == keyword {
            let mut check = idx + 1;

            while check < spl_len && match found_id {
                Some(i) =>
                    spls[check].len() > i + len && &spls[check][i..i+len] != end,
                None => !spls[check].contains(end)
            } {
                check += 1
            }

            let res = spls[idx+1..check]
                .iter()
                .map(|a| a.trim())
                .collect::<Vec<&str>>()
                .join("\n");

            let res_len = res.len();

            return res[if temp {
                    ..res_len - 8
                } else {..res_len}].to_string();
        }
    }

    String::new()
}
