use ethers::providers::{Http, Middleware, Provider};
use std::{thread, time::Duration};

async fn check_rpc_health(rpc_url: &str) -> Result<(), String> {
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|e| e.to_string())?;

    match provider.get_block_number().await {
        Ok(block_number) => {
            println!("✅ RPC is healthy! Current block: {}", block_number);
            Ok(())
        }
        Err(err) => {
            println!("❌ RPC health check failed: {}", err);
            Err(err.to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://eth.blockrazor.xyz"; // Replace with your RPC URL

    println!("Starting Ethereum RPC health checker...");
    println!("Monitoring RPC endpoint: {}", rpc_url);

    loop {
        match check_rpc_health(rpc_url).await {
            Ok(_) => (),
            Err(_) => println!("Will retry in 5 seconds..."),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
