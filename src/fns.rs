use reqwest::Response;

pub async fn auth_post_req(url: &str, auth: &str, body: String) -> Response {
    if body.is_empty() || is_valid_http(url) || auth.is_empty() {
        panic!("Empty body/url/token!");
    }

    reqwest::Client::new().post(url)
        .bearer_auth(auth)
        .body(body)
        .send()
        .await
        .unwrap()
}


pub async fn post_req(url: &str, body: String) -> Response {
    if is_valid_http(url) {
        panic!("Empty body/url!")
    }

    reqwest::Client::new().post(url)
        .body(body)
        .send()
        .await
        .unwrap()

}

pub async fn get_req(url: &str) -> String {
    if is_valid_http(url) {
        panic!("Empty url!")
    }

    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[inline]
pub fn is_valid_http(url: &str) -> bool {
    if url.starts_with("http://") || url.starts_with("https://") {
        true
    } else {
        false
    }
}