use bip39::{Mnemonic, Language, Seed}; 
use bitcoin::network::constants::Network;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey}; 
use bitcoin::{Address, PrivateKey, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use rayon::prelude::*;
use std::io::{self, Write};
use std::str::FromStr;
use rand::RngCore;
use rusty_leveldb::{DB, Options};
use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};

fn main() {
    let secp = Secp256k1::new();
    let network = Network::Bitcoin;
    let scan_count = Arc::new(AtomicU64::new(0));

    // Database Setup
    let mut opt = Options::default();
    opt.create_if_missing = true;
    let db = DB::open("master_db", opt).unwrap_or_else(|_| {
        DB::open("temp_db", Options::default()).expect("Fatal DB Error")
    });
    let shared_db = Arc::new(Mutex::new(db));

    println!("🦊 DREX FOXEL - MINI APP ENGINE READY!");

    loop {
        // Taasan ang batch size para mas mabilis
        (0..2000).into_par_iter().for_each(|_| {
            let mut entropy = [0u8; 16]; 
            rand::thread_rng().fill_bytes(&mut entropy);
            
            let mnemonic = match Mnemonic::from_entropy(&entropy, Language::English) {
                Ok(m) => m,
                Err(_) => return,
            };
            
            let current_total = scan_count.fetch_add(1, Ordering::SeqCst);
            
            // Stats reporting para sa Web App
            if current_total % 1000 == 0 {
                println!("STATS:{}|{}", current_total, &mnemonic.to_string()[..20]);
            }

            let seed = Seed::new(&mnemonic, "");
            let root = ExtendedPrivKey::new_master(network, seed.as_bytes()).unwrap();
            let paths = ["m/44'/0'/0'/0/0", "m/84'/0'/0'/0/0"];

            for path_str in paths {
                let path = DerivationPath::from_str(path_str).unwrap();
                let derived = root.derive_priv(&secp, &path).unwrap();
                let priv_key = PrivateKey::new(derived.private_key, network);
                let pubkey = PublicKey::from_private_key(&secp, &priv_key);

                let addr = match path_str {
                    "m/44'/0'/0'/0/0" => Address::p2pkh(&pubkey, network).to_string(),
                    "m/84'/0'/0'/0/0" => Address::p2wpkh(&pubkey, network).unwrap().to_string(),
                    _ => String::new(),
                };

                let found = {
                    let mut db_lock = shared_db.lock().unwrap(); 
                    db_lock.get(addr.as_bytes()).is_some()
                };

                if found {
                    // Magsesend ng alert sa file na babasahin ng Web App
                    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open("JACKPOT.txt") {
                        let _ = writeln!(file, "Seed: {}\nAddr: {}\n---", mnemonic.to_string(), addr);
                    }
                }
            }
        });
    }
}
