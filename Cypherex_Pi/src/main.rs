use ring::aead::*;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct PasswordEntry {
    service: String,
    encrypted_password: Vec<u8>,
}

async fn store_password_on_blockchain(entry: &PasswordEntry) -> Result<(), reqwest::Error> {
    let client = Client::new();
    client.post("https://your-blockchain-api.com/store") // Store the password on chain
        .json(entry)
        .send()
        .await?;
    Ok(())
}

async fn retrieve_password_from_blockchain(service: &str) -> Result<PasswordEntry, reqwest::Error> {
    let client = Client::new();
    let response = client.get(&format!("https://your-blockchain-api.com/retrieve/{}", service)) // Retrieve the password
        .send()
        .await?
        .json::<PasswordEntry>()
        .await?;
    Ok(response)
}

fn encrypt_password(key: &[u8], password: &str) -> Result<Vec<u8>, Unspecified> {
    let mut sealing_key = SealingKey::new(&UnboundKey::new(&AES_256_GCM, key)?, LessSafeKey::new)?;
    let mut nonce = [0u8; 12];
    SystemRandom::new().fill(&mut nonce)?;

    let mut in_out = password.as_bytes().to_vec();
    in_out.extend_from_slice(&[0u8; 16]); // Space for the tag
    sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)?;

    let mut encrypted_password = nonce.to_vec();
    encrypted_password.extend_from_slice(&in_out);
    Ok(encrypted_password)
}

fn decrypt_password(key: &[u8], encrypted_password: &[u8]) -> Result<String, Unspecified> {
    let (nonce, in_out) = encrypted_password.split_at(12);
    let mut opening_key = OpeningKey::new(&UnboundKey::new(&AES_256_GCM, key)?, LessSafeKey::new)?;
    let decrypted_data = opening_key.open_in_place(Aad::empty(), &mut in_out.to_vec())?;
    Ok(String::from_utf8(decrypted_data.to_vec()).unwrap())
}

#[tokio::main] 
async fn main() {
    // Generate a secure key
    let rng = SystemRandom::new();
    let mut key = [0u8; 32];
    rng.fill(&mut key).unwrap();

    // Example password
    let password = "my_secure_password";
    let service = "example_service";

    // Encrypt the password
    let encrypted_password = encrypt_password(&key, password).unwrap();

    // Create a password entry
    let entry = PasswordEntry {
        service: service.to_string(),
        encrypted_password: encrypted_password.clone(),
    };

    // Store the encrypted password on the blockchain
    store_password_on_blockchain(&entry).await.unwrap();

    // Retrieve the encrypted password from the blockchain
    let retrieved_entry = retrieve_password_from_blockchain(service).await.unwrap();

    // Decrypt the password
    let decrypted_password = decrypt_password(&key, &retrieved_entry.encrypted_password).unwrap();

    println!("Decrypted password: {}", decrypted_password);
}