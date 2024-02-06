use dotenv::dotenv;
use reqwest::blocking::{Client,ClientBuilder};
use std::env;

slint::slint! {
    export component App inherits Window {
        GridLayout {

        }
    }
}
// dotenv().ok();

// let api_key = env::var("BIBLE_API_KEY").expect("| Code error | failed to find the api key");


fn main() {
    let url: &str = "https://api.scripture.api.bible/v1/bibles";
    let http_client:Client = Client::new();
    let http_result = http_client.get(url).send();

    if http_result.is_ok() {
        println!("response ok {:#?}", http_result.ok().unwrap());
    } else {
        print!("PROGRESS!! failure but progress.")
    }
}
