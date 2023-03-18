use crate::collect_gen::concat_lines_exponent0;

pub fn template(mut html: String, js: String) -> String {
    let mut test_js = js;

    match html.find("$") {
        Some(a) => {
            let mut idx = a;

            while &html[idx..idx+1] != "<" {
                idx+=1;
            }

            let mut fall = a;
            let mut up = a;

            let mut pig = a;

            while &html[fall..fall+1] != "\n" {
                fall-=1
            }

            while &html[up..up+1] != "\n" {
                up+=1
            }

            while &html[pig..pig+1] != ">" {
                pig-=1
            }

            match html[fall..up].find("id=") {
                Some(au) => {
                    let mut init = au+5;

                    while &html[init..init+1] != "\"" {
                        init += 1
                    }

                    let mut fall_ = html[pig..a].split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|a| {a.to_string()})
                        .collect::<Vec<String>>();

                    fall_ = fall_[..fall_.len()-1].to_vec();

                    let id = &html[au+4..init+1];

                    let mut up_ = html[a..idx].split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>();

                    up_ = up_[1..].to_vec() ;

                    println!("{}", &html[a..idx]);

                    let a_ = concat_lines_exponent0(fall_);
                    let b: &str = &html[a+1..idx];
                    let c: String = concat_lines_exponent0(up_);

                    if a_ != "".to_string() && c != "".to_string() {
                        test_js = format!("{test_js}\ndocument.getElementById({id}).innerText={:?}\ndocument.getElementById({id}).innerText+={b}\nndocument.getElementById({id}).innerText+={:?}"
                            , a_, c)
                    }
                    else if a_ != "".to_string() {
                        test_js = format!("{test_js}\ndocument.getElementById({id}).innerText={:?}\ndocument.getElementById({id}).innerText+={b}", a_)
                    }
                    else if c != "".to_string() {
                        test_js = format!("{test_js}\ndocument.getElementById({id}).innerText={b}\ndocument.getElementById({id}).innerText+={:?}", c)
                    }
                    else {
                        test_js = format!("{test_js}\ndocument.getElementById({id}).innerText={b}")
                    }

                    html.replace_range(fall..up, "");

                    return test_js
                },
                None => panic!("id not found on templating element")
            }
        },
        None => todo!()
    }
}