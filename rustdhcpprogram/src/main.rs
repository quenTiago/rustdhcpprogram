use local_ip_address::local_ip;
use dns_lookup::lookup_addr; // On importe l'outil pour chercher les noms
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    // 1. Trouver mon IP
    let my_local_ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("Erreur IP : {}", e);
            return;
        }
    };
    println!("Mon IP est : {}", my_local_ip);

    let ip_str = my_local_ip.to_string();
    let parts: Vec<&str> = ip_str.split('.').collect();
    if parts.len() != 4 { return; }
    let subnet = format!("{}.{}.{}.", parts[0], parts[1], parts[2]);

    println!("Scan et recherche de noms sur {}0/24...", subnet);

    // Le canal transmet maintenant un duo : (IP, Nom)
    let (tx, rx) = mpsc::channel::<(String, String)>();

    // 2. Scan
    for i in 1..255 {
        let subnet_clone = subnet.clone();
        let tx_clone = tx.clone();
        let my_ip_clone = ip_str.clone();

        thread::spawn(move || {
            let target_ip_str = format!("{}{}", subnet_clone, i);
            
            // On ignore notre propre PC
            if target_ip_str == my_ip_clone { return; }

            // A. Le Ping (Détection)
            let output = Command::new("ping")
                .args(["-n", "1", "-w", "250", &target_ip_str])
                .output();

            if let Ok(out) = output {
                let s = String::from_utf8_lossy(&out.stdout);
                
                // Si ça répond (TTL présent)
                if out.status.success() && s.contains("TTL=") {
                    
                    // B. Recherche du NOM (Reverse DNS)
                    // On essaie de convertir la string IP en objet IP pour la fonction lookup
                    let hostname = if let Ok(ip_addr) = IpAddr::from_str(&target_ip_str) {
                        // On cherche le nom, sinon on met "Inconnu"
                        lookup_addr(&ip_addr).unwrap_or_else(|_| "Inconnu".to_string())
                    } else {
                        "Erreur IP".to_string()
                    };

                    // On envoie le résultat (IP + Nom)
                    tx_clone.send((target_ip_str, hostname)).unwrap();
                }
            }
        });
    }

    // On ferme le canal d'envoi principal
    drop(tx);

    println!("\n--- Appareils connectés ---");
    println!("{:<20} | {:<30}", "ADRESSE IP", "NOM DE L'APPAREIL");
    println!("{:-<20} | {:-<30}", "", ""); // Ligne de séparation

    // 3. Affichage
    for (ip, name) in rx {
        println!("{:<20} | {:<30}", ip, name);
    }
    println!("\nFin du scan.");
}