use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Response {
    pub private_telegram_id: String
}

#[derive(Deserialize)]
pub struct ApiResp {
    pub success: bool,
    pub response_code: i64,
    pub results: Response
}


#[tokio::main]
pub async fn fetch_user(user: &str) ->  ApiResp {
    let client = reqwest::Client::new();
    let fmt = format!("https://api.intellivoid.net/spamprotection/v1/lookup?query={}", user);
    let res = client.get(fmt)
        .header("User-Agent", "SpamProtectionBot-rs")
        .send()
        .await
        .unwrap()
        .json::<ApiResp>()
        .await
        .unwrap();
    return res
}
