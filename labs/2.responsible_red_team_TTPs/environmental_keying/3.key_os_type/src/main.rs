extern crate whoami;

use std::process::exit;

use whoami::{username, hostname, platform};


pub enum EnvCheck {
    // An enum that can service any of the environmental checks we want to do
    Username(String),
    Hostname(String),
    OSType(String)
}


fn payload() {
    
    #[cfg(unix)] {
        println!("[*] Writing to /tmp/foo.txt");
        std::fs::File::create("/tmp/foo.txt").expect("create failed");
    }

    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        use std::process::Command;
        let output = Command::new("cmd")
            .creation_flags(0x08000000)
            .arg("/c")
            .arg("notepad.exe")
            .output()
            .expect("Failed to execute process");
    }
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
        
        EnvCheck::OSType(o) => {
            println!("[*] Checking OS Type");
            println!("[*] OS Type Key: {o}");
            let current_platform: String = platform().to_string().to_lowercase();
            println!("[*] Current platform: {}", current_platform);
            return o == current_platform.as_str();
        }
    }
}

fn does_not_match() {
    println!("[-] No match, sorry :(");
    println!("[-] Exiting...");
    exit(0);
}


fn main() {
    
    let all_env_checks: Vec<EnvCheck> = vec![
        //EnvCheck::Hostname("dev-kde".to_string()),
        //EnvCheck::Username("husky".to_string())
        EnvCheck::OSType("windows".to_string())
    ];

    let key_check_result:  bool = all_env_checks
        .iter()
        .all(|env_check| check_env_keys(env_check));
    

    //If they match, trigger the payload
    //If not, exit
    if !key_check_result {
        does_not_match();
    } else {
        payload();
    }
}
