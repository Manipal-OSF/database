use base64::{engine::general_purpose, Engine};
use sha2::{Digest, Sha512};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Use a better hashing function?
    // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#password-hashing-algorithms
    let mut hasher = Sha512::new();

    let password = args.get(1).expect("Please enter a password");
    let salt = uuid::Uuid::new_v4().to_string().replace("-", "");

    hasher.update(format!("{password}{salt}"));
    let result = hasher.finalize();

    let hash = general_purpose::STANDARD.encode(result);
    println!("Password: {password}\nHash: {hash}\nSalt: {salt}");
}
