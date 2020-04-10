use ring::{agreement, rand};

pub fn example() -> Result<(), Box<dyn std::error::Error>> {
    let rng = rand::SystemRandom::new();

    info!("rng {:?}", rng);

    // This next line breaks WASM:
    // let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
    //
    // info!("SC: {:?}", my_private_key);
    //
    // // Make `my_public_key` a byte slice containing my public key. In a real
    // // application, this would be sent to the peer in an encoded protocol
    // // message.
    // let my_public_key = my_private_key.compute_public_key().unwrap();
    //
    // info!("PK!! {:?}", my_public_key);

    Ok(())
}
