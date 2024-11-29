use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::{thread, time::Duration};

fn check_rpc_health(rpc_url: &str) -> Result<(), String> {
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    match client.get_version() {
        Ok(version) => {
            println!("✅ RPC is healthy! Version: {}", version.solana_core);
            Ok(())
        }
        Err(err) => {
            println!("❌ RPC health check failed: {}", err);
            Err(err.to_string())
        }
    }
}

fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";

    println!("Starting Solana RPC health checker...");
    println!("Monitoring RPC endpoint: {}", rpc_url);

    loop {
        match check_rpc_health(rpc_url) {
            Ok(_) => (),
            Err(_) => println!("Will retry in 5 seconds..."),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
