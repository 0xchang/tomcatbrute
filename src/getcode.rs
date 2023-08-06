use reqwest::{header::{HeaderMap}, Client};
use std::{sync::Arc, time::Duration};
use reqwest::header::HeaderValue;

pub async fn get_status_code(url: Arc<String>, up_en_b64: &str) -> (u16, Option<HeaderValue>) {
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

    let resp = match resp {
        Ok(r)=>Some(r),
        Err(_)=>None
    };
    let resp = match resp {
        Some(r)=>{
            r
        },
        None=>todo!()
    };

    let mut cookies=None;
    let mut code=0;

    if let Some(cookie) = resp.headers().get("Set-Cookie") {
        cookies = cookie.clone().into();
    };
    if let Some(codes) = resp.status().into() {
        code = codes.into();
    }
    (code,cookies)
}

pub async fn check_code(result: (u16, Option<HeaderValue>), up: &str) -> Option<&str> {
    let (code,cookies) = result;
    let res = if code == 200 || code == 302 {
        println!("code is {}, success: {} , Cookie: {:#?}", code, up , cookies);
        Some(up)
    } else if code == 403 || code == 404 {
        println!("code is {}, stop...", code);
        std::process::exit(0);
    } else if code == 401 {
        None
    } else {
        None
    };
    res
}
