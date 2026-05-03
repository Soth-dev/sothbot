pub fn f<S: AsRef<str>>(t: S, u: &str) -> String {
    format!("<{}>{}</{}>", u, t.as_ref(), u)
}

pub fn m<S: AsRef<str>>(t: S) -> String {
    f(t, "code")
}

pub fn q<S: AsRef<str>>(t: S) -> String {
    f(t, "blockquote")
}
