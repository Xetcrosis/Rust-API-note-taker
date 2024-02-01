slint::slint! {
    export component App inherits Window {
        GridLayout {

        }

    }
}

fn main() {
    let app: App = App::new().unwrap().run().unwrap();
    let weak: Weak<App> = app.as_weak();
}

// use reqwest::header::{HeaderMap, AUTHORIZATION, HeaderValue, HeaderName};
// use serde::Deserialize;


// #[tokio::main]
// async fn smth() {
//     let request_url = format!("https://api.scripture.api.bible/v1/bibles")
//     let body = reqwest::get()
// }
