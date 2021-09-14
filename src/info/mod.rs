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

/* Here's where we start declaring the other methods for ease of use to get information easily. Like for example, instead of info.results.entity_type, one can simply do * info.get_type()
 */
impl ApiResp {
    pub fn get_bl(&self) -> bool {
        self.results.attributes.is_blacklisted
    }
    pub fn get_success(&self) -> bool {
        self.success
    }
    pub fn get_resp_code(&self) -> i16 {
        self.response_code
    }
    pub fn get_ptid(&self) -> String {
    self.results.private_telegram_id.clone()
    }
    pub fn get_type(&self) -> String {
        self.results.entity_type.clone()
    }
    pub fn get_original_ptid(&self) -> String {
        self.results.attributes.original_private_id.clone().unwrap_or("None".to_string())
    }
    pub fn get_flag(&self) -> String {
        self.results.attributes.blacklist_flag.clone().unwrap_or("None".to_string())
    }
    pub fn get_verified(&self) -> bool {
        self.results.attributes.intellivoid_accounts_verified
    }
    pub fn get_operator(&self) -> bool {
        self.results.attributes.is_operator
    }
    pub fn ger_agent(&self) -> bool {
        self.results.attributes.is_agent
    }
    pub fn get_user_whitelisted(&self) -> bool {
        self.results.attributes.is_whitelisted
    }
    pub fn get_official(&self) -> bool {
        self.results.attributes.is_official
    }
    pub fn get_bl_reason(&self) -> String {
        self.results.attributes.blacklist_reason.clone().unwrap_or("None".to_string())
    }
    pub fn get_potential(&self) -> bool {
        self.results.attributes.is_potential_spammer
    }
    pub fn get_flag_evade(&self) -> bool {
        self.get_flag() == "0xEVADE"        
    }
    pub fn get_flag_spam(&self) -> bool {
        self.get_flag() == "0xSPAM"
    }
    pub fn get_flag_scam(&self) -> bool {
        self.get_flag() == "0xSCAM"
    }
    pub fn get_flag_cacp(&self) -> bool {
        self.get_flag() == "0xCACP"
    }
    pub fn get_flag_sp(&self) -> bool {
        self.get_flag() == "0xSP"
    }
    pub fn get_flag_piracy(&self) -> bool {
        self.get_flag() == "0xPIRACY"
    }
    pub fn get_flag_namespam(&self) -> bool {
        self.get_flag() == "0xNAMESPAM"
    }
    pub fn get_flag_imper(&self) -> bool {
        self.get_flag() == "0xIMPER"
    }
    pub fn get_flag_raid(&self) -> bool {
        self.get_flag() == "0xRAID"
    }
    pub fn get_flag_massadd(&self) -> bool {
        self.get_flag() == "0xMASSADD"
    }
    pub fn get_flag_private(&self) -> bool {
        self.get_flag() == "0xPRIVATE"
    }
    pub fn get_spam_predict(&self) -> f64 {
        self.results.spam_prediction.spam_prediction.clone().unwrap_or(0.0)
    }
    pub fn get_ham_predict(&self) -> f64 {
        self.results.spam_prediction.ham_prediction.clone().unwrap_or(0.0)
    }
    pub fn get_lang(&self) -> String {
        self.results.language_prediction.language.clone().unwrap_or("None".to_string())
    }
    pub fn get_lang_probabiility(&self) -> f64 {
        self.results.language_prediction.probability.clone().unwrap_or(0.0)
    }
}
