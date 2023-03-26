use log::{error, info};
use rsa::{
    pkcs8::EncodePrivateKey, pkcs8::*, Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey,
};
use std::{
    fs::{read_to_string, write},
    path::Path,
    result::Result,
    str,
};

const SEPARATOR: &str = "\u{00A6}";

/// Encrypts message string using 4096-bit RSA
///
/// Returns a hex encoded string
///
/// # Examples
///
/// ```
/// use deaddrop_rs::encryption::encrypt;
///
/// let input = String::from("keep it secret, keep it safe");
/// let enc_string: String = encrypt(input);
/// ```
pub fn encrypt(msg: String, sender: String) -> String {
    file_check("encrypt");

    let mut rng = rand::thread_rng();
    let pub_key = get_public_key();

    let msg = add_sender(add_mac(msg), sender);

    let enc_msg = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes())
        .unwrap_or_else(|e| {
            error!("Problem encrypting message: {}", e);
            panic!("Could not encrypt message")
        });

    to_string(enc_msg)
}

/// Decrypts message that was encoded using a 4096-RSA key pair. Use in conjuction with `encrypt()`.
///
/// Given a hex encoded string, returns plaintext string
///
/// # Examples
///
/// ```
/// use deaddrop_rs::encryption::decrypt;
///
/// let secret_revealed = decrypt(enc_string);
/// println!("But you've only just arrived! I don't understand. -- Neither do I. {}", secret_revealed);
/// ```
pub fn decrypt(msg: String) -> Result<String, String> {
    let priv_key = get_private_key();
    let message = &to_u8(msg);

    let output = priv_key
        .decrypt(Pkcs1v15Encrypt, message)
        .unwrap_or_else(|e| {
            error!("Problem decrypting message: {}", e);
            panic!("Could not decrypt message")
        });

    let out = String::from_utf8(output).unwrap_or_else(|e| {
        error!("Problem decrypting message: {}", e);
        panic!("Could not decrypt message")
    });

    match mac_check(out) {
        Some(output) => Ok(output),
        None => Err(String::from("message integrity check failure")),
    }
}

fn to_u8(input: String) -> Vec<u8> {
    //! converts hex string into byte vector
    hex::decode(input).unwrap_or_else(|e| {
        error!("Problem decoding hex string to u8 vector: {}", e);
        panic!("Decryption failed")
    })
}

fn to_string(input: Vec<u8>) -> String {
    //! converts byte vector into hex string
    hex::encode(input)
}

fn get_private_key() -> RsaPrivateKey {
    //! converts private key from pem file
    let input = read_to_string("secret.pem").unwrap_or_else(|e| {
        error!("pem file read failure: {}", e);
        panic!("failed to read PEM file")
    });

    RsaPrivateKey::from_pkcs8_pem(input.as_str()).unwrap_or_else(|e| {
        error!("Problem getting private key from PEM file: {}", e);
        panic!("Could not read from PEM file")
    })
}

fn get_public_key() -> RsaPublicKey {
    //! returns public key
    let priv_key = get_private_key();
    priv_key.to_public_key()
}

fn create_pem() {
    //! creates a pem file using a 4096 bit RSA key pair
    let mut rng = rand::thread_rng();
    let bits = 4096;

    // let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap_or_else(|e| {
        error!("failed to generate a key: {}", e);
        panic!("key generation failure")
    });

    let pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::default())
        .unwrap_or_else(|e| {
            error!("PEM file failed to be created: {}", e);
            panic!("failed to create PEM file")
        });

    info!("PEM file created");

    write("secret.pem", pem).unwrap();
}

fn file_check(category: &str) {
    //! checks if there is a pem file in root directory
    //! creates one if not found and currently trying to encrypt
    //! fails and panics if not found and currently trying to decrypt
    match category {
        "encrypt" => {
            if !Path::new("secret.pem").exists() {
                create_pem();
            }
        }
        "decrypt" => {
            if !Path::new("secret.pem").exists() {
                error!("Tried to decrypt without PEM file");
                panic!("Missing PEM file")
            }
        }
        _ => (),
    }
}

fn add_mac(message: String) -> String {
    //! add a MAC to the message
    let mac = hex::encode_upper(&message);
    let mut msg = message;

    // split point with message on left portion of string
    msg.push_str(SEPARATOR);
    msg.push_str(&mac);
    msg
}

fn mac_check(message: String) -> Option<String> {
    //! make sure that the message was not changed
    let msg_with_mac: Vec<&str> = message.split(SEPARATOR).collect();

    let msg = msg_with_mac[0].to_string();
    let og_mac = msg_with_mac[1].to_string();
    let new_mac = hex::encode_upper(&msg);

    let sender = msg_with_mac[2].to_string();

    if new_mac == og_mac {
        Some(format!("(From {}): {}", sender, msg))
    } else {
        None
    }
}

fn add_sender(message: String, name: String) -> String {
    let mut msg = message;
    msg.push_str(SEPARATOR);
    msg.push_str(&name);
    msg
}
