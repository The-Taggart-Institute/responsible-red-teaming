extern crate whoami;

use std::process::exit;

use whoami::{username, hostname};


pub enum EnvCheck {
    // An enum that can service any of the environmental checks we want to do
    Username(String),
    Hostname(String)
}


fn payload() {
    // Placeholder for any kind of payload that we want to run
    println!("[+] Boom!");
}

fn check_env_keys(env: &EnvCheck) -> bool {
    // What ultimately evaluates our key and discovered value to determine if we're on target
    println!("[*] Checking key...");
    
    match env {
        EnvCheck::Username(u) => {
            println!("[*] Checking Username");
            println!("[*] Username key: {u}");
            let current_username: String = username().to_lowercase();
            println!("[*] Current username: {current_username}");
            return u == current_username.as_str()
        },
        
        EnvCheck::Hostname(h) => {
            println!("[*] Checking Hostname");
            println!("[*] Hostname key: {h}");
            let current_hostname: String = hostname().to_lowercase();
            println!("[*] Current hostname: {current_hostname}");
            return h == current_hostname.as_str()
        },
    }
}

fn does_not_match() {
    println!("[-] No match, sorry :(");
    println!("[-] Exiting...");
    // For this demo, we can exit. Imagine that you could put any kind of evasive move here, like self-deleting the binary with the houdini crate
	exit(0);
}


fn main() {
    // We create an Hostname enum with the keying value passed in as the string
    let hostname_check: EnvCheck = EnvCheck::Hostname("dev-kde".to_string());

    // And, if we want to do more checks, we can pass in multiple EnvCheck enums
    // let all_env_checks: Vec<EnvCheck> = vec![
    //     EnvCheck::Hostname("dev-kde".to_string()),
    //     EnvCheck::Username("husky".to_string())
    // ];


    //Check an individual env key
    let key_check_result = check_env_keys(&hostname_check);
    
    // OR... iterate through a vector of EnvCheck enums and return true ONLY if all checks pass
    // let key_check_result:  bool = all_env_checks
    //     .iter()
    //     .all(|env_check| check_env_keys(env_check));
    
    //If they match, trigger the payload
    //If not, exit
    if !key_check_result {
        does_not_match();
    } else {
        payload();
    }
}
