use crate::consts::NEW_LINE_CHAR;
use crate::std_err::ErrType::{LibraryError, NotFound};
use crate::std_err::StdErr;
use tinyget::get;

pub fn libs(name: &str, is_script: bool) -> String {
    let dir = if is_script {"scripts"} else {"libs"};
    let url = format!("https://raw.githubusercontent.com/tm-ahad/cream/master/{dir}/{name}");

    let resp = get(&url).send().unwrap_or_else(|e| {
        StdErr::exec(LibraryError, &e.to_string());
        todo!()
    });

    if resp.status_code == 200 {
        let mut res = resp.as_str().unwrap_or_else(|e| panic!("{e}")).to_string();
        res.push(NEW_LINE_CHAR);
        res
    } else {
        StdErr::exec(NotFound, &format!("Package {name} not found"));
        todo!()
    }
}

pub fn private_work_lib() -> String {
    libs("private_work_lib.js", true)
}
