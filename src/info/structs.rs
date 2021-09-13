use serde::Deserialize;

/* After importing serde to deserialize the JSON output, we move on to declaring the structs.
 * We took care of implementing the right types depending on the attributes.
 * There are some types which return null if a non-existent entity is passed.
 * We took care of this by using the Option<T> type in rust.
 */

#[derive(Deserialize, Debug, Default)]
pub struct Response {
    pub private_telegram_id: String,
    pub entity_type: String,
    pub attributes: Attributes,
    pub language_prediction: LangPredict,
    pub spam_prediction: SpamPredict,
    pub last_updated: i128,
}

#[derive(Deserialize, Debug, Default)]
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

#[derive(Deserialize, Debug, Default)]
pub struct LangPredict {
    pub language: Option<String>,
    pub probability: Option<f64>,
}

#[derive(Deserialize, Debug, Default)]
pub struct SpamPredict {
    pub ham_prediction: Option<f64>,
    pub spam_prediction: Option<f64>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Error {
    pub error_code: i8,
    pub r#type: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
    #[serde(default)]
    pub error: Error,
    pub success: bool,
    pub response_code: i16,
    #[serde(default)]
    pub results: Response,
}
