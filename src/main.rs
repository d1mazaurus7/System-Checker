use std::io::{self,Write};
use std::process::Command;
fn main() {

    println!("--- System Checker ---");

    let date = Command::new("date")
        .output()
        .expect("Failed to execute date...");
    println!("Date:\n{}", String::from_utf8_lossy(&date.stdout));

    let ip = Command::new("ip")
        .args(&["-br", "addr", "show", "wlan0"])
        .output()
        .expect("Failed to get ip");
    println!("Network:\n{}", String::from_utf8_lossy(&ip.stdout));

    let fail2ban = Command::new("sudo")
        .args(&["fail2ban-client", "status", "sshd"])
        .output()
        .expect("Failed");
    println!("Security:\n{}", String::from_utf8_lossy(&fail2ban.stdout));
   
    let logs = Command::new("sudo")
        .args(&["journalctl", "-p", "3", "-xb", "-n", "3", "--no-pager"])
        .output()
        .expect("Failed to get logs...");
    println!("Logs\n{}", String::from_utf8_lossy(&logs.stdout));
    println!("--- Done ---");
}
