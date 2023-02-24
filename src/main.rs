use std::{collections::HashMap, fs};

use reqwest::{
    header::{HeaderMap, ACCEPT, CONTENT_TYPE, COOKIE, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_yaml;

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded; charset=UTF-8"
            .parse()
            .unwrap(),
    );
    headers.insert(
        USER_AGENT,
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15"
            .parse()
            .unwrap());
    headers.insert(
        ACCEPT,
        "application/json, text/javascript, */*; q=0.01"
            .parse()
            .unwrap(),
    );
    headers
}

async fn login(
    client: &Client,
    headers: HeaderMap,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut request = HashMap::new();
    request.insert("email", config.email.as_str());
    request.insert("passwd", config.password.as_str());
    let res = client
        .post(format!("{}/auth/login", config.host))
        .form(&request)
        .headers(headers)
        .send()
        .await?;
    let mut cookies = res.headers().get_all("set-cookie").iter();
    let uid = cookies
        .next()
        .unwrap()
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap();
    let email = cookies
        .next()
        .unwrap()
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap();
    let key = cookies
        .next()
        .unwrap()
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap();
    let ip = cookies
        .next()
        .unwrap()
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap();
    let expire_in = cookies
        .next()
        .unwrap()
        .to_str()
        .unwrap()
        .split(";")
        .next()
        .unwrap();
    let cookie = format!("{};{};{};{};{}", uid, email, key, ip, expire_in);
    Ok(cookie)
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    email: String,
    password: String,
    host: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_str = fs::read_to_string("src/app.yaml")?;
    let config: Config = serde_yaml::from_str(yaml_str.as_str())?;
    println!("当前配置：{:?}", config);
    let client = reqwest::Client::new();
    let headers = headers();
    let cookie = login(&client, headers, &config).await?;
    println!("登录信息：{:?}", cookie);
    let res = client
        .post(format!("{}/user/checkin", config.host))
        .header(COOKIE, cookie)
        .send()
        .await?;
    println!("签到信息：{:?}", res.text().await?);
    Ok(())
}
