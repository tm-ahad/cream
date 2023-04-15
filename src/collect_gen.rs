pub fn collect_gen(toks: String, keyword: String, found_id: usize, end: &str) -> String {
    let splited_v = toks.split("\n").collect::<Vec<&str>>();
    let mut lines: Vec<String> = vec![];
    let mut _idx = 0;

    for (si, spl) in splited_v.clone().into_iter().enumerate() {
        let t = spl.replace(" ", "");

        if t.len() >= found_id + keyword.len() &&
           &t[found_id..found_id + keyword.len()] == keyword.as_str()  {

            for spl in &splited_v.clone()[si + 1..splited_v.len() - 1] {
                if spl == &"" {
                    continue;
                }

                while spl != &end {
                    lines.push(spl.trim().to_string());
                    break;
                }
            }
        }
        _idx += 1;
    }

    return concat_lines_exponent0(lines);
}

pub fn concat_lines_exponent0(lines: Vec<String>) -> String {
    let mut result = String::new();

    for l in lines.iter() {
        if l.trim() == "" {
            continue;
        }
        result = format!("{}\n{}", result, l);
    }
    return result;
}
