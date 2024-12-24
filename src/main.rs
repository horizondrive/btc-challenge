use std::io::stdin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bitcoin::{Address, Network, PublicKey};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::task;
use tokio::time::sleep;

const LOOKING_FOR: &str = "1BY8GQbnueYofwSuFAT3USAhGjPrkxDdW9";

async fn append_to_file(file_path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await?;
    file.write_all(format!("{}\n", data).as_bytes()).await?;
    Ok(())
}

pub async fn generate_random_btc_address(
    number: u64,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    task::spawn_blocking(move || {
        let mut secret_key_bytes = [0u8; 32];
        secret_key_bytes[24..].copy_from_slice(&number.to_be_bytes());
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(&secret_key_bytes)?;
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let public_key = PublicKey::new(bitcoin::secp256k1::PublicKey::from_secret_key(
            &secp,
            &secret_key,
        ));
        let address = Address::p2pkh(&public_key, Network::Bitcoin).to_string();

        Ok(address.eq(LOOKING_FOR))
    })
    .await?
}

#[tokio::main]
async fn main() {
    let range_start = 0x4000000000000000;
    let range_end = 0x7fffffffffffffff;
    let start_percentage = 80;
    let start_point = range_start + ((start_percentage as u64 * ((range_end - range_start) / 100)));

    println!("Please specify thread count: ");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let semaphore = Arc::new(Semaphore::new(s.parse::<usize>().expect("Not a number!")));
    let generated_counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = generated_counter.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            let count = counter_clone.load(Ordering::Relaxed);
            let at = start_point - count as u64;
            println!("Addresses generated so far: {}, at {}", count, at);
        }
    });

    let mut current_number = start_point;

    loop {
        let semaphore_permit = semaphore.clone().acquire_owned().await.unwrap();
        let counter_clone = generated_counter.clone();

        tokio::task::spawn(async move {
            let _permit = semaphore_permit;
            let current_number = current_number;

            if let Ok(found) = generate_random_btc_address(current_number.clone()).await {
                if found {
                    if let Err(e) = append_to_file("output.txt", &current_number.to_string()).await
                    {
                        eprintln!("Failed to write to output file: {}", e);
                        println!("NUMBER: {}", current_number); // print private key
                    }
                }

                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });

        current_number -= 1;
    }
}
