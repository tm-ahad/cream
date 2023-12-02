use crate::consts::NEW_LINE_CHAR;
use crate::std_err::ErrType::LibraryError;
use crate::std_err::StdErr;
use ureq::{get, Response};

pub fn libs(name: &str) -> String {
    let url = format!("https://raw.githubusercontent.com/tm-ahad/cream/master/libs/{name}");

    let resp = get(&url).call().unwrap_or_else(|e| {
        StdErr::exec(LibraryError, &e.to_string());
        Response::new(404, "PackageError", "").unwrap()
    });

    if resp.status() == 200 {
        let mut pack = resp.into_string().unwrap_or_else(|e| panic!("{e}"));
        pack.push(NEW_LINE_CHAR);
        pack
    } else {
        StdErr::exec(LibraryError, &format!("Package {name} not found"));
        todo!()
    }
}

pub fn private_work_lib() -> String {
    String::from("
class Work {
    #value;
    constructor(init) {
        this.#value = init;
    }

    do(then) {
        try {
            let _res = this.#value();

            let res = then({
                state: \"done\",
                error: null,
                value: _res
            });

            return res;
        } catch (e) {
           throw e;
        }
    }
}\n")
}
