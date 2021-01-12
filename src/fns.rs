use ureq::Response;

pub async fn auth_post_req(url: &str, auth: &str, body: String) -> Response {
    if body.is_empty() || !is_valid_http(url) || auth.is_empty() {
        panic!("Empty body/url/token!");
    }

    ureq::post(url)
        .set("X-Auth-Token", auth)
        .send_string(body.as_str())
        .unwrap()

}


pub async fn post_req(url: &str, body: String) -> Response {
    if !is_valid_http(url) {
        panic!("Empty body/url!")
    }

    ureq::post(url)
        .send_string(body.as_str())
        .unwrap()
}

pub async fn get_req(url: &str) -> Response {
    if !is_valid_http(url) {
        panic!("Empty url!")
    }

    ureq::get(url)
        .call()
        .unwrap()
}

#[inline]
pub fn is_valid_http(url: &str) -> bool {
    if url.starts_with("http://") || url.starts_with("https://") {
        true
    } else {
        false
    }
}