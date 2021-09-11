use serde::Deserialize;

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
