use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub private_telegram_id: String,
    pub entity_type: String,
    pub attributes: Attributes,
}

#[derive(Deserialize)]
pub enum Any {
    String,
    Null,
}

#[derive(Deserialize)]
pub struct Attributes {
    pub is_blacklisted: bool,
    pub blacklist_flag: Any,
    pub blacklist_reason: Any,
    pub orignal_private_id: Any,
    pub is_potential_spammer: bool,
    pub is_operator: bool,
    pub is_agent: bool,
    pub is_whitelisted: bool,
    pub intellivoid_accounts_verified: bool,
    pub is_official: bool,
}

#[derive(Deserialize)]
pub struct ApiResp {
    pub success: bool,
    pub response_code: i64,
    pub results: Response
}
