
extern crate ring;

fn aead() -> Result<(), ring::error::Unspecified> {

    use ring::{agreement, rand};
    use untrusted;

    let rng = rand::SystemRandom::new();

    let my_private_key =
        agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;

    // Make `my_public_key` a byte slice containing my public key. In a real
    // application, this would be sent to the peer in an encoded protocol
    // message.
    let mut my_public_key = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let my_public_key =
        &mut my_public_key[..my_private_key.public_key_len()];
    my_private_key.compute_public_key(my_public_key)?;

    // In a real application, the peer public key would be parsed out of a
    // protocol message. Here we just generate one.
    let mut peer_public_key_buf = [0u8; agreement::PUBLIC_KEY_MAX_LEN];
    let peer_public_key;
    {
        let peer_private_key =
            agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)?;
        peer_public_key =
            &mut peer_public_key_buf[..peer_private_key.public_key_len()];
        peer_private_key.compute_public_key(peer_public_key)?;
    }
    let peer_public_key = untrusted::Input::from(peer_public_key);

    // In a real application, the protocol specifies how to determine what
    // algorithm was used to generate the peer's private key. Here, we know it
    // is X25519 since we just generated it.
    let peer_public_key_alg = &agreement::X25519;

    agreement::agree_ephemeral(my_private_key, peer_public_key_alg,
                               peer_public_key, ring::error::Unspecified,
                               |_key_material| {
                                   // In a real application, we'd apply a KDF to the key material and the
                                   // public keys (as recommended in RFC 7748) and then derive session
                                   // keys from the result. We omit all that here.
                                   Ok(())
                               });

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn a() {
        // * [] Establish Authenticated Encryption over a stream.
    }

    #[test]
    fn b() {
        // * [] Write *Matchmaker* server to store Public key, ip address pairs.
    }

    #[test]
    fn c() {
        // * [] *Publish* a Public key, ip address pair to `Matchmaker`.
    }

    #[test]
    fn d() {
        // * [] *Lookup* an ip address from matchmaker.
    }
}

