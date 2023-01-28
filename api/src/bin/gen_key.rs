use base64::{engine::general_purpose, Engine};
use rsa::{errors::Error, Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

fn main() -> Result<(), Error> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 4096)?;
    let public_key = RsaPublicKey::from(&private_key);

    let data = b"Hello";
    let raw = public_key.encrypt(&mut rng, Pkcs1v15Encrypt::default(), data)?;

    let api_key = general_purpose::STANDARD.encode(raw);

    println!("{api_key}");

    let a = general_purpose::STANDARD.decode(api_key).unwrap();
    let b = private_key.decrypt(Pkcs1v15Encrypt::default(), a.as_slice())?;

    assert_eq!(data, b.as_slice());
    Ok(())
}
