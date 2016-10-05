use verex::*;

pub fn is_url(url: &str) -> bool {
    let mut verex = Verex::new();
    let regex = verex.maybe("http")
        .maybe("s")
        .maybe("://")
        .maybe("www.")
        .range(vec![('a', 'z'), ('0', '9')])
        .find(".")
        .range(vec![('a', 'z'), ('0', '9'), ('/', '/'), ('.', '.'), ('?', '?'), ('=', '='),
                    ('@', '@'), ('%', '%')])
        .compile()
        .unwrap();
    regex.is_match(url)
}
