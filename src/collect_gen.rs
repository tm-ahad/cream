
pub fn collect_gen(toks: String, keyword: String, end: &str, found_id: Option<usize>, temp: bool) -> String {
    let spls = toks.split("\n").collect::<Vec<&str>>();
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

            return spls[idx+1..if !temp {check} else {
                check-1
            }]
                .into_iter()
                .map(|a| a.trim())
                .collect::<Vec<&str>>()
                .join("\n");
        }
    }

    String::new()
}
