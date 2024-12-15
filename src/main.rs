use std::hash::Hash;
use std::io::stdin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::{collections::HashSet, sync::Arc};

use bitcoin::secp256k1::{rand, PublicKey as SecpPublicKey, Secp256k1};
use bitcoin::{Address, Network, PublicKey};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tokio::{sync::RwLock, task};

type AddressShard = Arc<RwLock<HashSet<String>>>;
type AddressSet = Vec<AddressShard>;

const NUM_SHARDS: usize = 32;

fn create_sharded_set() -> AddressSet {
    (0..NUM_SHARDS)
        .map(|_| Arc::new(RwLock::new(HashSet::new())))
        .collect()
}

fn get_shard<'a>(address_set: &'a AddressSet, address: &'a str) -> &'a AddressShard {
    let hash = {
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
        address.hash(&mut hasher);
        hasher.finish()
    };
    &address_set[hash as usize % NUM_SHARDS]
}

async fn load_addresses(file_paths: Vec<String>) -> AddressSet {
    let addresses = create_sharded_set();

    let mut tasks = vec![];

    for file_path in file_paths {
        let addresses_clone = addresses.clone();
        tasks.push(tokio::spawn(async move {
            if let Ok(contents) = tokio::fs::read_to_string(&file_path).await {
                for line in contents.lines() {
                    let shard = get_shard(&addresses_clone, line);
                    let mut lock = shard.write().await;
                    lock.insert(line.to_string());
                }
            } else {
                eprintln!("Failed to read file: {}", file_path);
            }
        }));
    }

    futures::future::join_all(tasks).await;

    addresses
}

async fn address_exists(addresses: &AddressSet, address: &str) -> bool {
    let shard = get_shard(addresses, address);
    let lock = shard.read().await;
    lock.contains(address)
}

async fn append_to_file(
    file_path: &str,
    d: &str,
    data: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await?;
    file.write_all(format!("{}, {}\n", d, data).as_bytes())
        .await?;
    Ok(())
}

pub async fn generate_random_btc_address(
) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
    task::spawn_blocking(move || {
        // Generate a random private key
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let bitcoin_public_key =
            PublicKey::from(SecpPublicKey::from_slice(&public_key.serialize()[..]).unwrap());

        let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin).to_string();

        let a = secret_key.secret_bytes();
        let b = hex::encode(a);

        Ok((b, address))
    })
    .await?
}

#[tokio::main]
async fn main() {
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
    let output_file = "output.txt";

    let file_paths = vec![
        "data/addresses_1.txt".to_string(),
        "data/addresses_2.txt".to_string(),
        "data/addresses_3.txt".to_string(),
        "data/addresses_4.txt".to_string(),
        "data/addresses_5.txt".to_string(),
        "data/addresses_6.txt".to_string(),
        "data/addresses_7.txt".to_string(),
        "data/addresses_8.txt".to_string(),
        "data/addresses_9.txt".to_string(),
        "data/addresses_10.txt".to_string(),
        "data/addresses_11.txt".to_string(),
        "data/addresses_12.txt".to_string(),
        "data/addresses_13.txt".to_string(),
        "data/addresses_14.txt".to_string(),
        "data/addresses_15.txt".to_string(),
        "data/addresses_16.txt".to_string(),
        "data/addresses_17.txt".to_string(),
        "data/addresses_18.txt".to_string(),
        "data/addresses_19.txt".to_string(),
        "data/addresses_20.txt".to_string(),
        "data/addresses_21.txt".to_string(),
    ];

    println!("Reading bitcoin addresses. Please wait, this can take a minute");
    let addresses = load_addresses(file_paths).await;
    println!("Loaded");

    let generated_counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = generated_counter.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(10)).await;
            let count = counter_clone.load(Ordering::Relaxed);
            println!("Addresses generated so far: {}", count);
        }
    });

    loop {
        let addresses_clone = addresses.clone();
        let semaphore_permit = semaphore.clone().acquire_owned().await.unwrap();
        let counter_clone = generated_counter.clone();

        tokio::task::spawn(async move {
            let _permit = semaphore_permit;

            if let Ok((private_key, new_address)) = generate_random_btc_address().await {
                if address_exists(&addresses_clone, &new_address).await {
                    if let Err(e) = append_to_file(output_file, &new_address, &private_key).await {
                        eprintln!("Failed to write to output file: {}", e);
                        println!("PRIVATE: {}", private_key); // print private key
                    }
                }

                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
    }
}
