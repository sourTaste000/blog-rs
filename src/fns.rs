use reqwest::Response;

pub async fn auth_post_req(url: &str, auth: &str, body: &str) -> Response {
    if body.is_empty() || is_valid_http(url) || auth.is_empty() {
        panic!("Empty body/url/token!");
    }

    let req = reqwest::Client::new().post(url)
        .bearer_auth(auth)
        .body(body)
        .send()
        .await
        .unwrap();

    return req;
}


pub async fn post_req(url: &str, body: &str) -> Response {
    if is_valid_http(url) {
        panic!("Empty body/url!")
    }

    let req = reqwest::Client::new().post(url)
        .body(body)
        .send()
        .await
        .unwrap();

    return req;
}

pub async fn get_req(url: &str) -> String {
    if is_valid_http(url) {
        panic!("Empty url!")
    }

    let req = reqwest::get(url).await.unwrap().text().await.unwrap();

    return req
}

#[inline]
pub fn is_valid_http(url: &str) -> bool {
    if url.starts_with("http://") || url.starts_with("https://") {
        true
    } else {
        false
    }
}