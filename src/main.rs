use clap::Parser;
use chrono::{Timelike, Utc};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::fs::{File, write};
use std::path::Path;
use hex;
use std::str;

mod aes;
use aes::{cipher, inv_cipher};


const KEY: &str = "makesmthgpplwant";

// Simple program to manage passwords locally
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Create new password
    #[arg(short, long)]
    new: Option<String>,

    /// Url for a new password
    #[arg(short, long, requires = "new")]
    url: Option<String>,
    
    /// View all passwords
    #[arg(short, long)]
    view: Option<bool>,

    /// Delete a password
    #[arg(short, long)]
    delete: Option<String>,
}

fn add_new_password(password: String, url: String) {
    assert!(password.len() > 0, "Password cannot be empty");
    assert!(url.len() > 0, "Url cannot be empty");

    let path = Path::new("data.txt");
    if !path.exists() {
        println!("Creating data.txt");
        let mut file = File::create(path)
            .expect("Failed to create file");
        
        file.write_all("ciphertext, url, date\n"
            .as_bytes())
            .expect("Unable to write data");
        println!("data.txt created successfully");
    }
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Failed to open file");

    let ciphertext: String = encrypt(&password);
    file.write_all(format!("{}, {}, {:02}:{:02}:{:02} {}\n", 
            ciphertext, 
            url, 
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" })
            .as_bytes())
        .expect("Failed to write to file");
    println!("Password {password} added successfully | Cyphertext: {ciphertext}");
}

fn view_all_passwords() {
    assert!(OpenOptions::new().read(true).open("data.txt").is_ok(), "File does not exist");

    let content = read_to_string("data.txt").expect("Unable to read file");
    let mut lines: Vec<&str> = content.split("\n").collect();
    println!("{: <10} | {: <20} | {: <10}", "PASSWORD", "URL", "DATE ADDED");
    println!("{:-<10} | {:-<20} | {:-<10}", "", "", "");

    for line in lines {
        let columns: Vec<&str> = line.split(",").collect();
        if columns[0] == "ciphertext" { continue;}
        if columns[0] != "" {
            let ciphertext: String = columns[0].trim().to_string();
            let url: String = columns[1].trim().to_string();
            let date: String = columns[2].trim().to_string();
            let password: String = decrypt(ciphertext).expect("REASON");
            println!("{password} | {url} | {date}")
        }

    }

}

fn delete_password(url: String) {
    assert!(url.len() > 0, "Password cannot be empty");

    let content = read_to_string("data.txt").expect("Unable to read file");
    let mut lines: Vec<&str> = content.split("\n").collect();
    let mut new_content: String = String::new();

    let mut count = 0;
    for line in lines {
        let columns: Vec<&str> = line.split(",").collect();
        if columns.len() > 1 && columns[1].trim().contains(&url) {
            println!("Deleted {}", line);
            count += 1;
            continue;
        }
        new_content.push_str(line);
        new_content.push_str("\n");  

    }
    if count == 0 {
        println!("No password found for {url}")
    } 
    // Write the new content to the file
    write("data.txt", new_content).expect("Unable to write file");
    view_all_passwords()
}

fn encrypt(input: &str) -> String {
    assert_eq!(KEY.len(), 16, "Key length must be exactly 16 bytes");

    // Convert key string to bytes
    let mut key: [u8; 16] = [0u8; 16];
    let key_len = KEY.len();
    key[..key_len].copy_from_slice(KEY.as_bytes());

    // Convert input string to bytes
    let mut input_bytes: [u8; 16] = [0u8; 16];
    let input_len = input.len();
    input_bytes[..input_len].copy_from_slice(input.as_bytes());

    // Encrypt the input using AES
    let mut encrypted: [u8; 16] = [0u8; 16];
    cipher(&input_bytes, &mut encrypted, &key);

    return hex::encode(encrypted);
}

fn decrypt(ciphertext: String) -> Result<String, ()> {
    assert_eq!(KEY.len(), 16, "Key length must be exactly 16 bytes");

    // Convert key string to bytes
    let mut key: [u8; 16] = [0u8; 16];
    let key_len = KEY.len();
    key[..key_len].copy_from_slice(KEY.as_bytes());


    let encrypted_bytes = hex::decode(&ciphertext).expect("Failed to decode hex");
    let mut encrypted_array: [u8; 16] = [0; 16];
    encrypted_array.copy_from_slice(&encrypted_bytes);
    
    // Decrypt the bytes using AES
    let mut decrypted: [u8; 16] = [0u8; 16];
    inv_cipher(&encrypted_array, &mut decrypted, &key);
    
    match String::from_utf8(decrypted.to_vec()) {
        Ok(decrypted_str) => Ok(decrypted_str),
        Err(e) => {
            println!("Error converting decrypted bytes to string: {}", e);
            Err(())
        }
    }
    
}


fn main() {

    let args: Args = Args::parse();
    println!("When adding new passwords, make sure to escape (\\) special characters!\n");

    if let Some(new) = args.new {
        if let Some(url) = args.url {
            println!("New: {} / {}", new, url);
            add_new_password(new, url);    
        }
    } else if let Some(view) = args.view {
        if view { view_all_passwords(); }
    } else if let Some(delete) = args.delete {
        delete_password(delete);
    } else {
        println!("No arguments provided");
    }

}

