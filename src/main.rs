use std::io::{self,Write};
use std::process::Command;
fn main() {
    loop {
        println!("\n[--- System Checker ---]");
        println!("Hello! Hi! I'll check your system, display logs, and information about your PC.\n Select an option: ");
        println!("1. Show date");
        println!("2. Show Network");
        println!("3. Show security status (Fail2ban)");
        println!("4. Show critical errors (Logs)");
        println!("5. Show Hardware Info (CPU/GPU)");
        println!("q. Quit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Input error...");
        let choice = choice.trim();

        match choice {
            "1" => {
                let out = Command::new("date")
                .output()
                .expect("Failed to show date...");
                println!("Date:\n{}", String::from_utf8_lossy(&out.stdout));
            }

            "2" => {
                let out = Command::new("ip")
                .args(&["-br", "addr", "show", "wlan0"])
                .output()
                .expect("Failed to show Network...");
                println!("Network:\n{}", String::from_utf8_lossy(&out.stdout));
            }

            "3" => {
                let out = Command::new("sudo")
                 .args(&["fail2ban-client", "status", "sshd"])
                 .output()
                 .expect("Failed");
                 println!("Security status:\n{}", String::from_utf8_lossy(&out.stdout));                            
            }
      
            "4" => {
                let out = Command::new("sudo")
                .args(&["journalctl", "-p", "3", "-xb", "-n", "3", "--no-pager"])
                .output()
                .expect("Failed to get logs...");
                println!("Logs\n{}", String::from_utf8_lossy(&out.stdout));
            }
            
            "5" => {
                println!("--- Hardware Information ---");
                let cpu = Command::new("sh")
                .args(&["-c", "grep 'model name' /proc/cpuinfo | head -n 1 | cut -d: -f2"])
                .output()
                .expect("Failed to get CPU info");
    
                let gpu = Command::new("sh")
                .args(&["-c", "lspci | grep -i vga | cut -d: -f3"])
                .output()
                .expect("Failed to get GPU info");

                let mem = Command::new("free")
                .arg("-h")
                .output()
                .expect("Failed to get RAM info");

                println!("CPU: {}", String::from_utf8_lossy(&cpu.stdout).trim());
                println!("GPU: {}", String::from_utf8_lossy(&gpu.stdout).trim());
                println!("RAM:\n{}", String::from_utf8_lossy(&mem.stdout));

            }

                _ => {
               println!("Fail...");
        }   }

    }
}    
