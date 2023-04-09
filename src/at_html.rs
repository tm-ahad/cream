use crate::gen_id::gen_id;
use crate::scope::Pair;
use crate::state_base::_StateBase;
use crate::v8_parse::v8_parse;
use rusty_v8::{ContextScope, HandleScope};
use std::string::String;

pub fn at_html(
    mut html: String,
    js: String,
    scope: &mut ContextScope<HandleScope>,
    base: &mut _StateBase,
) -> Pair {
    while html.contains("@html") {
        match html.find("@html") {
            Some(a) => {
                let mut idx = a + 6;
                let mut pig = a;

                while &html[idx..idx + 1] != "\""
                    && &html[idx..idx + 1] != "<"
                    && &html[idx..idx + 1] != " "
                {
                    idx += 1
                }

                let mut id_x = a;
                let mut js = String::new();

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
                

                while &html[pig..pig + 1] != ">" {
                    pig -= 1
                }

                let id = match html.find("id=\"") {
                    Some(a) => {
                        let mut end = a;

                        while &html[end..end + 1] != "\"" {
                            end += 1;
                        }

                        html[a + 4..end].to_string()
                    }
                    None => gen_id(),
                };

                let val = html[a + 5..idx].to_string();

                let result = &*v8_parse(scope, &*val);
                
                base._set(
                    val.clone(),
                    format!("{js}\ndocument.getElementById({:?}).innerHTML", id,),
                    val.clone(),
                );

                html.insert_str(pig, &*format!(" id=\"{}\"", id));
                let len = id.len() + 6;

                html.replace_range(a + len..idx + len, result);
            },
            None => {}
        }
    }

    Pair(html, js)
}
