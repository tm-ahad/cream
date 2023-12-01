use crate::consts::NEW_LINE_CHAR;
use crate::std_err::ErrType::PackageError;
use crate::std_err::StdErr;
use ureq::{get, Response};

pub fn libs(name: &str) -> String {
    let url = format!("https://raw.githubusercontent.com/tm-ahad/cream/master/libs/{name}");

    let resp = get(&url).call().unwrap_or_else(|e| {
        StdErr::exec(PackageError, &e.to_string());
        Response::new(404, "PackageError", "").unwrap()
    });

    return if resp.status() == 200 {
        let mut pack = resp.into_string().unwrap_or_else(|e| panic!("{e}"));
        pack.push(NEW_LINE_CHAR);
        pack
    } else {
        StdErr::exec(PackageError, &format!("Package {pkg} not found"));
        todo!()
    }
}
