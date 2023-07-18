
pub enum Matcher<'a> {
    Component(&'a String),
    Template,
    Dom,
    Sin,
    Cam
}

impl<'a> ToString for Matcher<'a> {
    fn to_string(&self) -> String {
        String::from(match self {
            Matcher::Template => "<temp>",
            Matcher::Cam => "cam",
            Matcher::Sin => "sin",
            Matcher::Dom => "dom",
            Matcher::Component(s) => return format!("{s} {}", "{"),
        })
    }
}

impl<'a> Matcher<'a> {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Matcher::Template => "<temp>",
            Matcher::Cam => "cam",
            Matcher::Sin => "sin",
            Matcher::Dom => "dom",
            Matcher::Component(s) => s,
        }
    }
}
