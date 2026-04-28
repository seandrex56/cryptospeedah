use bip39::Mnemonic;
use bitcoin::network::constants::Network;
use bitcoin::bip32::{ExtendedPrivKey, DerivationPath};
use bitcoin::{Address, PrivateKey, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use rayon::prelude::*;
use std::io::Write;
use std::str::FromStr;
use rand::RngCore;
use rusty_leveldb::{DB, Options};
use std::sync::{Arc, Mutex}; // Kailangan natin 'to para sa Thread Safety

fn main() {
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;

    // 1. Buksan ang DB at i-wrap sa Arc at Mutex
    let opt = Options::default();
    let db = DB::open("master_db", opt).expect("Ayaw bumukas ng master_db!");
    
    // Ang Arc ay para mapasa sa threads, ang Mutex ay para sa safe access
    let shared_db = Arc::new(Mutex::new(db));
    
    println!("[OK] LevelDB Connected. Hunting Mode: ON.");

    loop {
        (0..100).into_par_iter().for_each(|_| {
            let mut entropy = [0u8; 16]; 
            rand::thread_rng().fill_bytes(&mut entropy);
            
            let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
            let seed = mnemonic.to_seed("");
            let root = ExtendedPrivKey::new_master(network, &seed).unwrap();

            let paths = [
                "m/44'/0'/0'/0/0", 
                "m/49'/0'/0'/0/0", 
                "m/84'/0'/0'/0/0", 
            ];

            for path_str in paths {
                let path = DerivationPath::from_str(path_str).unwrap();
                let derived = root.derive_priv(&secp, &path).unwrap();
                let priv_key = PrivateKey::new(derived.private_key, network);
                let pubkey = PublicKey::from_private_key(&secp, &priv_key);

                let addr = match path_str {
                    "m/44'/0'/0'/0/0" => Address::p2pkh(&pubkey, network).to_string(),
                    "m/49'/0'/0'/0/0" => Address::p2shwpkh(&pubkey, network).unwrap().to_string(),
                    "m/84'/0'/0'/0/0" => Address::p2wpkh(&pubkey, network).unwrap().to_string(),
                    _ => String::new(),
                };

                // 2. Thread-Safe DB Lookup
                // 'Lock' muna natin ang DB para makapag-search
                let mut db_lock = shared_db.lock().unwrap(); 
                let found = db_lock.get(addr.as_bytes()).is_some();
                drop(db_lock); // Release ang lock agad para sa ibang threads

                if found {
                    eprintln!("\n\n[!!!] JACKPOT BOSS! [!!!]");
                    eprintln!("SEED: {}", mnemonic.to_string());
                    eprintln!("ADDR: {}", addr);
                    
                    let mut jackpot_file = std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("JACKPOT.txt")
                        .unwrap();
                    writeln!(jackpot_file, "Seed: {}\nAddr: {}\nPath: {}\n---", mnemonic.to_string(), addr, path_str).unwrap();
                }
            }
            
            // Print matrix view (optional)
            println!("{}", mnemonic.to_string());
        });
    }
}
