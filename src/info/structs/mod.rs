use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub private_telegram_id: String,
    pub entity_type: String,
    pub attributes: Attributes,
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
pub struct ApiResp {
    pub success: bool,
    pub response_code: i64,
    pub results: Response
}
