use crate::scope::Pair;

pub fn at_html(mut html: String, mut js: String) -> Pair {
    while html.contains("@html") {
        match html.find("@html") {
            Some(a) => {
                let mut idx = a + 7;

                while &html[idx..idx + 1] != "\""
                    && &html[idx..idx + 1] != "<"
                    && &html[idx..idx + 1] != " "
                {
                    idx += 1
                }

                let mut id_x = a;

                while &html[id_x..id_x + 1] != "\"" {
                    if id_x == 1 {
                        panic!("Id expected at templating element")
                    }

                    id_x -= 1
                }

                let mut is_x = id_x;

                while &html[is_x - 4..is_x] != "id=\"" {
                    is_x -= 1
                }

                let mut val = &html[a + 6..idx + 1];
                let lk = val.len();

                if &val[lk - 1..lk] == "<" {
                    val = &val[0..(idx - (a + 6))]
                }

                js = format!(
                    "{js}\ndocument.getElementById({:?}).innerHTML={}",
                    &html[is_x..id_x],
                    val
                );

                html.replace_range(a..idx + 1, "")
            }
            None => {
                panic!("Yout computer messsed up the thing as well")
            }
        }
    }

    Pair(html, js)
}
