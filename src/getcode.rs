use reqwest::{header::HeaderMap, Client};
use std::{sync::Arc, time::Duration};

pub async fn get_status_code(url: Arc<String>, up_en_b64: &str) -> u16 {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent",
     "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.72 Safari/537.36".parse().unwrap());
    headers.insert(
        "Authorization",
        ("Basic ".to_owned() + up_en_b64).parse().unwrap(),
    );
    let c = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let resp = c.get(&*url).send().await;
    let code = match resp {
        Ok(r) => r.status().into(),
        Err(_) => 0,
    };
    code
}

pub async fn check_code(code: u16, up: &str) -> Option<&str> {
    let res = if code == 200 || code == 302 {
        println!("code is {},success: {}", code, up);
        Some(up)
    } else if code == 403 || code == 404 {
        println!("code is {}, stop...", code);
        None
    } else if code == 401 {
        println!("code is 401, wait...");
        None
    } else {
        None
    };
    res
}
