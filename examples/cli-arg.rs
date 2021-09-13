use spamprotection::info;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg: String = args[1].parse().expect("invalid account");

    if info::get_success(&arg) {
        println!("You have successfully connected to Intellivoid API!\n");
    } else {
        println!("This entity is not in Intellivoid database or there may be a problem with the API at this moment.");
        process::exit(1);
    }

    if info::get_resp(&arg) == 200 {
        println!("It was a successful response!\n");
    }

    if info::get_bl(&arg) {
        println!(
            "This {} is blacklisted due to {} with the {} flag.\n",
            info::get_type(&arg),
            info::get_reason(&arg),
            info::get_flag(&arg)
        );
    } else {
        println!("This {} is not blacklisted!\n", info::get_type(&arg));
    }
    if info::get_flag_evade(&arg) {
        println!(
            "Beware! This {} has an evading flag. This is the original PTID of the person: {}",
            info::get_type(&arg),
            info::get_original_ptid(&arg)
        );
    }

    if info::get_potential(&arg) {
        println!(
            "Beware! This {} is a potential spammer!!",
            info::get_type(&arg)
        );
    }

    if info::get_user_verified(&arg) {
        println!("This {} is verified by Intellivoid!", info::get_type(&arg));
    }

    if info::get_flag_raid(&arg) {
        println!("Beware! This {} has a raiding flag.", info::get_type(&arg));
    }

    if info::get_flag_spam(&arg) {
        println!("Beware! This {} has a spammer flag.", info::get_type(&arg));
    }

    if info::get_flag_scam(&arg) {
        println!("Beware! This {} has a scammer flag.", info::get_type(&arg));
    }

    if info::get_flag_private(&arg) {
        println!("Beware! This {} has a private flag.", info::get_type(&arg));
    }

    if info::get_flag_sp(&arg) {
        println!("Beware! This {} has a special flag.", info::get_type(&arg));
    }

    if info::get_flag_namespam(&arg) {
        println!("Beware! This {} has a namespam flag.", info::get_type(&arg));
    }

    if info::get_flag_imper(&arg) {
        println!(
            "Beware! This {} has a impersonation flag.",
            info::get_type(&arg)
        );
    }

    if info::get_flag_massadd(&arg) {
        println!(
            "Beware! This {} has a mass adding flag.",
            info::get_type(&arg)
        );
    }

    if info::get_flag_piracy(&arg) {
        println!("Beware! This {} has a piracy flag.", info::get_type(&arg));
    }

    if info::get_flag_cacp(&arg) {
        println!(
            "Beware! This {} has a child abuse flag.",
            info::get_type(&arg)
        );
    }

    // You can also alternatively use the full() method in info module
    // This is however tedious and not suggested.
    let info = info::full(&arg);

    println!("{:#?}", info);
}
