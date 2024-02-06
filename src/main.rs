use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::env;

use serde::Deserialize;
slint::slint! {
    export component App inherits Window {
        GridLayout {

        }
    }
}

async fn grab_verses() -> HeaderMap {
    dotenv().ok();

    let api_key = env::var("BIBLE_API_KEY").expect("| Code error | failed to find the api key");

    let api_grab: String = format!("api-key: {}", api_key);

    let mut headers = HeaderMap::new();
    headers.insert(
        "api-key",
        HeaderValue::from_str(&api_grab).expect("Invalid header value"),
    );
    let res = reqwest::ge
}

fn main() {

}
