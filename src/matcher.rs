use std::fmt::Display;

pub enum Matcher<'a> {
    Component(&'a str),
    Template,
}

impl<'a> Display for Matcher<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Matcher::Template => String::from("<temp>"),
            Matcher::Component(s) => format!("{s} {}", "{"),
        };

        write!(f, "{}", str)
    }
}

impl<'a> Matcher<'a> {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Matcher::Template => "<temp>",
            Matcher::Component(s) => s,
        }
    }
}
