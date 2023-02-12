use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, pkcs8::EncodePrivateKey, pkcs8::*};
use log::{info, error};
use std::{fs::{write,read_to_string}, path::Path, str};

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
pub fn encrypt (msg: String) -> String {
    file_check("encrypt");

    let mut rng = rand::thread_rng();

    let pub_key = get_public_key();

    let enc_msg = match pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, msg.as_bytes()) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Problem encrypting message: {}", e);
            panic!("Could not encrypt message")
        }
    };
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
pub fn decrypt(msg: String) -> String {
    let priv_key = get_private_key();
    let message = &to_u8(msg);
    let output = match priv_key.decrypt(Pkcs1v15Encrypt, message) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Problem decrypting message: {}", e);
            panic!("Could not decrypt message")
        }
    };

    match String::from_utf8(output) {
        Ok(out) => out,
        Err(e) => {
            error!("Problem decrypting message: {}", e);
            panic!("Could not decrypt message")
        }
    }
}

fn to_u8(input: String) -> Vec<u8> {
    //! converts hex string into byte vector
    match hex::decode(input) {
        Ok(out) => out,
        Err(e) => {
            error!("Problem decoding hex string to u8 vector: {}", e);
            panic!("Decryption failed")
        }
    }
}

fn to_string(input: Vec<u8>) -> String {
    //! converts byte vector into hex string
    hex::encode(input)
}

fn get_private_key() -> RsaPrivateKey {
    //! converts private key from pem file
    let input = read_to_string("secret.pem").unwrap();
    match RsaPrivateKey::from_pkcs8_pem(input.as_str()) {
        Ok(key) => key,
        Err(e) => {
            error!("Problem getting private key from pem file: {}", e);
            panic!("Could not read from pem file")
        }
    }
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
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pem = private_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::default()).unwrap();

    info!("PEM file created");
    
    write("secret.pem",pem).unwrap();
}

fn file_check(category: &str) {
    //! checks if there is a pem file in root directory
    //! creates one if not found and currently trying to encrypt
    //! fails and panics if not found and currently trying to decrypt
    match category {
        "encrypt" => if !Path::new("secret.pem").exists() {
            create_pem();
        },
        "decrypt" => if !Path::new("secret.pem").exists() {
            error!("Tried to decrypt without PEM file");
            panic!("Missing PEM file")
        },
        _ => ()
    }
}