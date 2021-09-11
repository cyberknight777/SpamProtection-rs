mod structs;
use structs::ApiResp;

#[tokio::main]
pub async fn full<T: std::fmt::Display>(user: T) -> ApiResp {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.intellivoid.net/spamprotection/v1/lookup?query={}", user))
        .header("User-Agent", "SpamProtectionBot-rs")
        .send()
        .await
        .unwrap()
        .json::<ApiResp>()
        .await
        .unwrap();
    return res
}
