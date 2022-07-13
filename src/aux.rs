use std::process::exit;
use std::process::Command;

use regex::Regex;

pub fn ping(ip: &String) -> String {
    let cmd = Command::new("ping")
        .args(["-c", "1", ip])
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&cmd.stdout)
        .to_string()
}

pub fn get_ttl(output: &String) -> u8 {
    let re1 = Regex::new(r"ttl=\d{1,3}").unwrap();
    let re2 = Regex::new(r"\d+").unwrap();

    if re1.is_match(output) == false {
        println!("[-] Host doesn't exist or is down");
        exit(2);
    }

    let ttl_msg = re1.find(output)
        .unwrap()
        .as_str();

    re2.find(ttl_msg)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .unwrap()    
}

pub fn check_os(ttl: &u8, ip: &String)  {
    match ttl {
        0..=64   => println!("[+] {} is Linux (ttl => {})", ip, ttl),
        65..=128 => println!("[+] {} is Windows (ttl => {})", ip, ttl),
        _ => println!("[+] {} is neither Linux nor Windows (ttl => {})", ip, ttl)
    }
}
