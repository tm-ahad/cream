use crate::state_base::_StateBase;
use crate::cpu_error::cpu_error;
use std::string::String;

pub struct Pair(pub String, pub String);

pub fn _scope(mut html: String, mut js: String, st: &mut _StateBase) -> Pair {
    while html.contains('{') && html.contains('}') {
        match html.find('{') {
            Some(a) => {
                html.replace_range(a..a + 1, "");
                let mut f = a;

                while &html[f..f + 1] != "}" {
                    f += 1
                }

                let mut fin = html[a..f + 1].to_string();

                return match fin.find('$') {
                    Some(au) => {
                        let mut pig = au;

                        while &fin[pig..pig + 1] != "`" {
                            pig -= 1
                        }

                        fin.insert(pig, ' ');

                        let mut zig = au + 1;
                        let mut vend = au + 1;

                        let mut idx = au;

                        while &fin[zig..zig + 1] != "`" {
                            zig += 1
                        }

                        while &fin[vend..vend + 1] != " " && &fin[vend..vend + 1] != "`" {
                            vend += 1
                        }

                        let start = &fin[pig + 1..au];
                        let end = &fin[vend..zig];

                        let mut op_end = pig;

                        while &fin[op_end..op_end + 1] != "{" {
                            op_end -= 1;
                        }

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

                        match html[fall..up].find("id=") {
                            Some(b) => {
                                let mut init = b + 5;

                                while &html[init..init + 1] != "\"" {
                                    init += 1
                                }

                                let c = &fin[op_end + 1..pig];

                                let val = &fin[au + 2..vend];
                                let id = &html[b + 4..init];

                                let mut changer = String::new();

                                if !start.is_empty() {
                                    changer.push_str(&format!(
                                        "document.getElementById({:?}){}={:?};",
                                        id, c, start
                                    ))
                                }
                                changer.push_str(&format!(
                                    "document.getElementById({:?}).innerHTML{}={};",
                                    id, c, val
                                ));

                                if !end.is_empty() {
                                    changer.push_str(&format!(
                                        "document.getElementById({:?}).innerHTML{}={:?};",
                                        id, c, end
                                    ))
                                }

                                let mut cn = fin.clone();

                                cn.replace_range(pig + 1..vend + 1, changer.as_str());

                                js = format!("{js}\n{cn}");
                                let mut yu = html.clone();

                                st._set(
                                    val.to_string(),
                                    format!(
                                        "document.getElementById({:?}).innerHTML{}={:?};",
                                        id, c, end
                                    ),

                                    val.to_string()
                                );

                                yu.replace_range(a..f + 2, "");

                                Pair(js, yu)
                            }
                            None => return Pair(js, html),
                        }
                    }
                    None => Pair(js, html),
                };
            }

            None => cpu_error(),
        }
    }

    Pair(js, html)
}
