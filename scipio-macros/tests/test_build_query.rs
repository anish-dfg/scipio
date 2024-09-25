use scipio_macros::ToQueryString;

struct S {
    pub field: String,
    pub direction: String,
}

impl std::fmt::Display for S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.field, self.direction)
    }
}

#[derive(ToQueryString)]
struct Query {
    a: String,
    b: bool,
    c: ::core::option::Option<String>,
    d: std::option::Option<bool>,
    e: ::std::option::Option<Vec<String>>,
    f: Option<Vec<bool>>,
    g: Option<Vec<S>>,
    h: Vec<bool>,
}

pub fn main() {
    let q = Query {
        a: "a".to_owned(),
        b: true,
        c: Some("c".to_owned()),
        d: Some(false),
        e: Some(vec!["e".to_owned(), "E".to_owned()]),
        f: None,
        g: Some(vec![
            S { field: "g1".to_owned(), direction: "asc".to_owned() },
            S { field: "g2".to_owned(), direction: "desc".to_owned() },
        ]),
        h: vec![true, false, true],
    };

    let qs = q.to_query_string();
    dbg!(qs);
}
