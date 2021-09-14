use spamprotection::info;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: String = args[1].parse().expect("invalid account");

    let info = info::full(&arg);

    if info.get_success() {
        println!("You have successfully connected to Intellivoid API!\n");
    } else {
        println!("This entity is not in Intellivoid database or there may be a problem with the API at this moment.");
        process::exit(1);
    }

    if info.get_resp_code() == 200 {
        println!("It was a successful response!\n");
    }

    if info.get_bl() {
        println!(
            "This {} is blacklisted due to {} with the {} flag.\n",
            info.get_type(),
            info.get_bl_reason(),
            info.get_flag()
        );
    } else {
        println!("This {} is not blacklisted!\n", info.get_type());
    }
    if info.get_flag_evade() {
        println!(
            "Beware! This {} has an evading flag. This is the original PTID of the person: {}",
            info.get_type(),
            info.get_original_ptid()
        );
    }

    if info.get_potential() {
        println!(
            "Beware! This {} is a potential spammer!!",
            info.get_type()
        );
    }

    if info.get_verified() {
        println!("This {} is verified by Intellivoid!", info.get_type());
    }

    if info.get_flag_raid() {
        println!("Beware! This {} has a raiding flag.", info.get_type());
    }

    if info.get_flag_spam() {
        println!("Beware! This {} has a spammer flag.", info.get_type());
    }

    if info.get_flag_scam() {
        println!("Beware! This {} has a scammer flag.", info.get_type());
    }

    if info.get_flag_private() {
        println!("Beware! This {} has a private flag.", info.get_type());
    }

    if info.get_flag_sp() {
        println!("Beware! This {} has a special flag.", info.get_type());
    }

    if info.get_flag_namespam() {
        println!("Beware! This {} has a namespam flag.", info.get_type());
    }

    if info.get_flag_imper() {
        println!(
            "Beware! This {} has a impersonation flag.",
            info.get_type()
        );
    }

    if info.get_flag_massadd() {
        println!(
            "Beware! This {} has a mass adding flag.",
            info.get_type()
        );
    }

    if info.get_flag_piracy() {
        println!("Beware! This {} has a piracy flag.", info.get_type());
    }

    if info.get_flag_cacp() {
        println!(
            "Beware! This {} has a child abuse flag.",
            info.get_type()
        );
    }
}
