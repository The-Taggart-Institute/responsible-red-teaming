extern crate whoami;

use std::process::exit;
use whoami::{username};


fn payload() {
    // Placeholder for any kind of payload that we want to run
    println!("[+] Boom!");
}

fn check_env_keys() -> bool {
    println!("Checking keys...");

    // For this demo, assume a static value for the username
    let key_username: String = "husky".to_string();
    // This value can be hard coded in the code here or passed in via arguments.

    let current_username = username().to_lowercase();
    println!("[+] Username: {}", current_username);
    println!("[+] Looking for: {}", key_username);
    
    // Evaluate the keying criteria, return true or false
    current_username == key_username    
}

fn does_not_match() {
    println!("[-] No match, sorry :(");
    println!("[-] Exiting...");
    exit(0);
}


fn main() {
    //Check the env keys
    let key_check_result = check_env_keys();

    //If they match, trigger the payload
    //If not, exit
    if !key_check_result {
        does_not_match();
    } else {
        payload()
    }
}
