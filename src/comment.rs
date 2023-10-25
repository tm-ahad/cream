use crate::consts::{NEW_LINE, NIL};

pub fn comment(app: &mut String) {
    while let Some(a) = app.find("//") {
        let mut n = a + 2;

        while &app[n..n + 1] != NEW_LINE {
            n += 1;
        }

        app.replace_range(a..n + 1, NIL)
    }

    while let Some(a) = app.find("/*") {
        let mut n = a + 2;

        while &app[n..n + 2] != "*/" {
            n += 1;
        }

        app.replace_range(a..n + 2, NIL)
    }
}
