mod structs;
use structs::ApiResp;

/*  Now that we have declared the structs.rs file to be loaded as a rust module, We will move on to declaring the full() method.
 *  What this full() method essentially does is that it makes the initial POST request to the API using the reqwest library with a form called query with its value  *   *  being the userID/username/PTID provided by the user of the library. The json output is also parsed with serde and the Display trait is applied for cleaner output.
 */
pub fn full<T: std::fmt::Display + serde::Serialize>(user: T) -> ApiResp {
    reqwest::blocking::Client::new()
        .post("https://api.intellivoid.net/spamprotection/v1/lookup")
        .header("User-Agent", "SpamProtectionBot-rs")
        .form(&[("query", user)])
        .send()
        .unwrap()
        .json::<ApiResp>()
        .unwrap()
}

/* Here's where we start declaring the other methods for ease of use to get information easily. Like for example, instead of info.results.entity_type, one can simply do * info::get_type
 */

pub fn get_flag<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user)
        .results
        .attributes
        .blacklist_flag
        .unwrap_or("None".to_string())
}

pub fn get_success<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).success
}

pub fn get_resp<T: std::fmt::Display + serde::Serialize>(user: T) -> i16 {
    full(user).response_code
}

pub fn get_bl<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_blacklisted
}

pub fn get_type<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user).results.entity_type
}

pub fn get_original_ptid<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user)
        .results
        .attributes
        .original_private_id
        .unwrap_or("None".to_string())
}

pub fn get_user_verified<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.intellivoid_accounts_verified
}

pub fn get_user_operator<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_operator
}

pub fn get_user_agent<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_agent
}

pub fn get_user_whitelisted<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_whitelisted
}

pub fn get_user_official<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_official
}

pub fn get_reason<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user)
        .results
        .attributes
        .blacklist_reason
        .unwrap_or("None".to_string())
}

pub fn get_potential<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    full(user).results.attributes.is_potential_spammer
}

/* Now here we start declaring methods for specific blacklist flags.
 * This makes it simpler to check if a user is blacklisted with a specific flag or not.
 * Note that we manually make an if expression to parse the output and the boolean values.
 */

pub fn get_flag_evade<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xEVADE"
}

pub fn get_flag_spam<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xSPAM"
}

pub fn get_flag_scam<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xSCAM"
}

pub fn get_flag_cacp<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xCACP"
}

pub fn get_flag_sp<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xSP"
}

pub fn get_flag_piracy<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xPIRACY"
}

pub fn get_flag_namespam<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xNAMESPAM"
}

pub fn get_flag_massadd<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xMASSADD"
}

pub fn get_flag_imper<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xIMPER"
}

pub fn get_flag_raid<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xRAID"
}

pub fn get_flag_private<T: std::fmt::Display + serde::Serialize>(user: T) -> bool {
    get_flag(user) == "0xPRIVATE"
}

pub fn get_spam_predict<T: std::fmt::Display + serde::Serialize>(user: T) -> f64 {
    full(user)
        .results
        .spam_prediction
        .spam_prediction
        .unwrap_or(0.0)
}

pub fn get_ham_predict<T: std::fmt::Display + serde::Serialize>(user: T) -> f64 {
    full(user)
        .results
        .spam_prediction
        .ham_prediction
        .unwrap_or(0.0)

}

pub fn get_lang<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user)
        .results
        .language_prediction
        .language
        .unwrap_or("None".to_string())
}

pub fn get_lang_probability<T: std::fmt::Display + serde::Serialize>(user: T) -> f64 {
    full(user)
        .results
        .language_prediction
        .probability
        .unwrap_or(0.0)
}

pub fn get_ptid<T: std::fmt::Display + serde::Serialize>(user: T) -> String {
    full(user).results.private_telegram_id
}
