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

pub fn get_flag<T: std::fmt::Display>(user: T) -> Option<String> {
    return full(user).results.attributes.blacklist_flag
}

pub fn get_success<T: std::fmt::Display>(user: T) -> bool {
    return full(user).success
}

pub fn get_resp<T: std::fmt::Display>(user: T) -> i16 {
    return full(user).response_code
}

pub fn get_bl<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_blacklisted
}

pub fn get_type<T: std::fmt::Display>(user: T) ->  String {
    return full(user).results.entity_type
}

pub fn get_original_ptid<T: std::fmt::Display>(user: T) -> Option<String> {
    return full(user).results.attributes.original_private_id
}

pub fn get_user_verified<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.intellivoid_accounts_verified
}

pub fn get_user_operator<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_operator
}

pub fn get_user_agent<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_agent
}

pub fn get_user_whitelisted<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_whitelisted
}

pub fn get_user_official<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_official
}

pub fn get_reason<T: std::fmt::Display>(user: T) -> Option<String> {
    return full(user).results.attributes.blacklist_reason
}

pub fn get_potential<T: std::fmt::Display>(user: T) -> bool {
    return full(user).results.attributes.is_potential_spammer
}

pub fn get_flag_evade<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xEVADE" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_spam<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xSPAM" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_scam<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xSCAM" {
	return true;
    } else {
	return false;
    }
}


pub fn get_flag_cacp<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xCACP" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_sp<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xSP" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_piracy<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xPIRACY" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_namespam<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xNAMESPAM" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_massadd<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xMASSADD" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_imper<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xIMPER" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_raid<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xRAID" {
	return true;
    } else {
	return false;
    }
}

pub fn get_flag_private<T: std::fmt::Display>(user: T) -> bool {
    if full(user).results.attributes.blacklist_flag.unwrap_or("None".to_string()) == "0xPRIVATE" {
	return true;
    } else {
	return false;
    }
}

pub fn get_spam_predict<T: std::fmt::Display>(user: T) ->  Option<f64> {
    return full(user).results.spam_prediction.spam_prediction
}

pub fn get_ham_predict<T: std::fmt::Display>(user: T) -> Option<f64> {
    return full(user).results.spam_prediction.ham_prediction
}

pub fn get_lang<T: std::fmt::Display>(user: T) ->  Option<String> {
    return full(user).results.language_prediction.language
}

pub fn get_probability<T: std::fmt::Display>(user: T) -> Option<f64> {
    return full(user).results.language_prediction.probability
}
