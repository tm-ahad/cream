use std::string::String;

pub struct Pair(pub String, pub String);

impl Pair {
    pub fn new() -> Self { Pair(String::new(), String::new())  }
}

pub fn scope(mut html: String, mut js: String) -> Pair {
    while html.contains("{") && html.contains("}") {
         match html.find("{") {
            Some(a) => {
                html.replace_range(a..a + 1, "");
                let mut f = a;

                while &html[f..f + 1] != "}" {
                    f += 1
                }

                let fin = format!("\n{}\n", &html[a..f + 1]);

                match fin.find("$") {
                    Some(au) => {
                        let mut zig = au;
                        let mut vend = au;

                        let mut pig = au;
                        let mut cn = html[a..f+1].to_string();

                        let mut idx = au;

                        while &fin[zig..zig + 1] != "`" {
                            zig += 1
                        }

                        while &fin[vend..vend+1] != " " {
                            vend += 1
                        }

                        while &fin[pig..pig + 1] != "`" {
                            pig -= 1
                        }

                        let start = &fin[pig+1..au];
                        let end = &fin[vend..zig];

                        while &html[idx..idx + 1] != " " {
                            idx += 1;
                        }

                        let mut fall = a;
                        let mut up = a;

                        while &html[fall..fall + 1] != "\n" {
                            fall -= 1
                        }

                        while &html[up..up + 1] != "\n" {
                            up += 1
                        }

                        return match html[fall..up].find("id=") {
                            Some(b) => {
                                let mut init = b + 5;

                                while &html[init..init + 1] != "\"" {
                                    init += 1
                                }

                                let c = &fin[pig-1..pig];
                                let val = &fin[au+1..vend];
                                let id = &html[b + 4..init];

                                let mut changer = String::new();

                                if !start.is_empty() {
                                    changer.push_str(&*format!(
                                        "document.getElementById({:?}){}={:?};",
                                        id, c, start
                                    ))
                                }
                                changer.push_str(&*format!(
                                    "document.getElementById({:?}){}={};",
                                    id, c, val
                                ));

                                if !end.is_empty() {
                                    changer.push_str(&*format!(
                                        "document.getElementById({:?}){}={:?};",
                                        id, c, end
                                    ))
                                }

                                cn.replace_range(
                                    pig-2..zig,
                                    changer.as_str()
                                );

                                js = format!("{js}\n{cn}");
                                let mut yu = html.clone();

                                yu.replace_range(a..f+2, "");

                                Pair(js, yu)
                            }
                            None => Pair::new()
                        }
                    }
                    None => return Pair::new()
                }
            }

            None => panic!("Ram fucked up ! "),
        }
    }

    Pair::new()
}
