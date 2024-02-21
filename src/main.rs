use dotenv::dotenv;
use reqwest::{blocking::{Client,ClientBuilder}, header};
use std::env;

slint::slint! {
    export component App inherits Window {
        GridLayout {

        }
    }
}
// dotenv().ok();



fn main() {
    let api_key: String = env::var("BIBLE_API_KEY").expect("| Code error | failed to find the api key");
    let url: &str = "https://api.scripture.api.bible/v1/bibles";
    let http_client:Client = Client::new();
    let http_result = http_client.get(url).send();
    headers.insert("API-key", header::HeaderValue::fromstr(api_key).unwrap());

    if http_result.is_ok() {
        println!("response ok {:#?}", http_result.ok().unwrap());
    } else {
        print!("It dont do")
    }
}
