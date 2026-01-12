use local_ip_address::local_ip;
use dns_lookup::lookup_addr;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::net::IpAddr;
use std::str::FromStr;
use std::io::{self, Write}; // <--- Nouveaux imports pour gérer l'entrée/sortie

fn main() {
    // 1. Trouver mon IP
    let my_local_ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Erreur IP : {}", e);
            pause_exit(); // On pause aussi en cas d'erreur
            return;
        }
    };
    println!("Mon IP est : {}", my_local_ip);

    let ip_str = my_local_ip.to_string();
    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() != 4 { 
        pause_exit();
        return; 
    }
    let subnet = format!("{}.{}.{}.", parts[0], parts[1], parts[2]);

    println!("Scan et recherche de noms sur {}0/24...", subnet);

    let (tx, rx) = mpsc::channel::<(String, String)>();

    // 2. Scan (Multithread)
    for i in 1..255 {
        let subnet_clone = subnet.clone();
        let tx_clone = tx.clone();
        let my_ip_clone = ip_str.clone();

        thread::spawn(move || {
            let target_ip_str = format!("{}{}", subnet_clone, i);
            
            if target_ip_str == my_ip_clone { return; }

            let output = Command::new("ping")
                .args(["-n", "1", "-w", "250", &target_ip_str])
                .output();

            if let Ok(out) = output {
                let s = String::from_utf8_lossy(&out.stdout);
                
                if out.status.success() && s.contains("TTL=") {
                    let hostname = if let Ok(ip_addr) = IpAddr::from_str(&target_ip_str) {
                        lookup_addr(&ip_addr).unwrap_or_else(|_| "Inconnu".to_string())
                    } else {
                        "Erreur IP".to_string()
                    };
                    tx_clone.send((target_ip_str, hostname)).unwrap();
                }
            }
        });
    }

    drop(tx);

    println!("\n--- Appareils connectés ---");
    println!("{:<20} | {:<30}", "ADRESSE IP", "NOM DE L'APPAREIL");
    println!("{:-<20} | {:-<30}", "", "");

    for (ip, name) in rx {
        println!("{:<20} | {:<30}", ip, name);
    }
    
    println!("\nFin du scan.");

    // --- PAUSE FINALE ---
    pause_exit();
}

// Petite fonction pour gérer la pause proprement
fn pause_exit() {
    print!("\nAppuyez sur Entrée pour fermer...");
    io::stdout().flush().unwrap(); // Force l'affichage du texte tout de suite
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
}