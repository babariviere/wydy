use url::Url;

pub fn is_url(url: &str) -> bool {
    let mut url = url.to_owned();
    if !url.starts_with("http") {
        url = format!("http://{}", url);
    }
    Url::parse(&url).is_ok()
}
