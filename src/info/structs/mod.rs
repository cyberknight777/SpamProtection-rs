use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub private_telegram_id: String,
    pub entity_type: String,
    pub attributes: Attributes,
    pub language_prediction: LangPredict,
    pub spam_prediction: SpamPredict,
    pub last_updated: i128
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub is_blacklisted: bool,
    pub blacklist_flag: Option<String>,
    pub blacklist_reason: Option<String>,
    pub original_private_id: Option<String>,
    pub is_potential_spammer: bool,
    pub is_operator: bool,
    pub is_agent: bool,
    pub is_whitelisted: bool,
    pub intellivoid_accounts_verified: bool,
    pub is_official: bool,
}

#[derive(Deserialize, Debug)]
pub struct LangPredict {
    pub language: String,
    pub probability: f64
}

#[derive(Deserialize, Debug)]
pub struct SpamPredict {
    pub ham_prediction: f64,
    pub spam_prediction: f64
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
    pub success: bool,
    pub response_code: i16,
    pub results: Response
}
