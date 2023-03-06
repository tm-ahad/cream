pub fn collect_gen(toks: String, keyword: String, found_id: usize, end: &str) -> String {
    let splited_v = toks.split("\n").collect::<Vec<&str>>();
    let mut lines: Vec<&str> = vec![];
    let mut _idx = 0;

    for (si, spl) in splited_v.clone().into_iter().enumerate() {

        if spl.len() >= found_id + keyword.len() {


            println!("{} - {}", &spl.trim(), keyword);

            if spl[found_id..found_id + keyword.len()].trim() == keyword.as_str() {

                for spl in &splited_v.clone()[si + 1..splited_v.len() - 1] {

                    if spl == &"" {
                        continue
                    }

                    while spl.trim() != end {
                        lines.push(spl);
                        break;
                    }
                }
            }
        }
        _idx += 1;
    }

    return concat_lines_exponent0(lines);
}

pub fn concat_lines_exponent0(lines: Vec<&str>) -> String {
    let mut idx = 0;
    let mut result = String::new();
    let len = lines.len() - 1;

    for l in lines.iter() {
        if idx == len {
            result = format!("{}{}", result, l);
        } else {
            result = format!("{}\n{}", result, l)
        }
        idx += 1
    }
    return result;
}
