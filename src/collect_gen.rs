pub fn collect_gen(toks: String, keyword: String, end: &str, found_id: usize) -> String {
    let spls = toks.split('\n').collect::<Vec<&str>>();
    let mut idx = 0;
    let len = end.len();

    for l in spls.to_vec() {
        if l.replace(' ', "") == keyword {
            let mut check = idx + 1;

            while spls[check].len() > found_id + len &&
                &spls[check][found_id..found_id+len] != end {

                check += 1
            }

            return spls[idx+1..check].join("\n");
        }

        idx += 1;
    }

    String::new()
}
