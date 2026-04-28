use bip39::{Mnemonic, Language, MnemonicType};
use warp::Filter;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Port setup para sa Render
    let health_route = warp::path::end().map(|| "BRUTE FORCE ENGINE: RUNNING");
    
    tokio::spawn(async {
        println!("Server gising sa port 10000...");
        warp::serve(health_route).run(([0, 0, 0, 0], 10000)).await;
    });

    println!("ENGINE STARTING... 24/7 MODE ACTIVATED.");

    loop {
        // Generate random 12 words
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        
        // Ang request mong format boss
        println!("WALLET CHECK: {}", mnemonic.phrase());

        // Konting hinga para hindi ma-ban sa Render
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
