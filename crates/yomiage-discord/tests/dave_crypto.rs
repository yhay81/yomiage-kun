use openmls_basic_credential::SignatureKeyPair;
use openmls_rust_crypto::RustCrypto;
use openmls_traits::{
    crypto::OpenMlsCrypto,
    signatures::Signer,
    types::{
        AeadType, CryptoError, HpkeAeadType, HpkeCiphertext, HpkeConfig, HpkeKdfType, HpkeKemType,
        SignatureScheme,
    },
};
use serde_json::Value;

fn bytes(value: &Value) -> Vec<u8> {
    hex::decode(value.as_str().unwrap()).unwrap()
}

// RFC 6979 A.2.5: fixed P-256/SHA-256 key and DER-encoded "sample" signature.
#[test]
fn p256_signatures_match_rfc6979_and_reject_tampering() {
    let private =
        hex::decode("c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721").unwrap();
    let public = hex::decode(concat!(
        "04",
        "60fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb6",
        "7903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299"
    ))
    .unwrap();
    let expected = hex::decode(concat!(
        "3046022100efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716",
        "022100f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8"
    ))
    .unwrap();
    let scheme = SignatureScheme::ECDSA_SECP256R1_SHA256;
    let crypto = RustCrypto::default();
    assert_eq!(crypto.sign(scheme, b"sample", &private).unwrap(), expected);
    let signer = SignatureKeyPair::from_raw(scheme, private, public.clone());
    assert_eq!(signer.sign(b"sample").unwrap(), expected);
    crypto.verify_signature(scheme, b"sample", &public, &expected).unwrap();
    assert!(crypto.verify_signature(scheme, b"tampered", &public, &expected).is_err());
    assert!(crypto.verify_signature(scheme, b"sample", &[0; 65], &expected).is_err());
    assert!(crypto.verify_signature(scheme, b"sample", &public, &expected[..20]).is_err());
}

// RFC 8032 section 7.1 TEST 1 also exercises the upgraded Ed25519 branch.
#[test]
fn ed25519_signatures_match_rfc8032() {
    let private =
        hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60").unwrap();
    let public =
        hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a").unwrap();
    let expected = hex::decode(concat!(
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155",
        "5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
    ))
    .unwrap();
    let scheme = SignatureScheme::ED25519;
    let crypto = RustCrypto::default();
    assert_eq!(crypto.sign(scheme, b"", &private).unwrap(), expected);
    assert_eq!(
        SignatureKeyPair::from_raw(scheme, private, public.clone()).sign(b"").unwrap(),
        expected
    );
    crypto.verify_signature(scheme, b"", &public, &expected).unwrap();
    assert!(crypto.verify_signature(scheme, b"tampered", &public, &expected).is_err());
}

// CFRG's RFC 9180 vectors, Base/P-256/HKDF-SHA256/AES128GCM (DAVE's HPKE suite).
#[test]
fn hpke_matches_external_key_derivation_decryption_and_export_vectors() {
    let vector: Value = serde_json::from_str(include_str!("fixtures/hpke-p256-base.json")).unwrap();
    let config =
        || HpkeConfig(HpkeKemType::DhKemP256, HpkeKdfType::HkdfSha256, HpkeAeadType::AesGcm128);
    let crypto = RustCrypto::default();
    let pair = crypto.derive_hpke_keypair(config(), &bytes(&vector["ikmR"])).unwrap();
    assert_eq!(&pair.private[..], bytes(&vector["skRm"]));
    assert_eq!(pair.public, bytes(&vector["pkRm"]));
    let encryption = &vector["encryptions"][0];
    let mut ciphertext = HpkeCiphertext {
        kem_output: bytes(&vector["enc"]).into(),
        ciphertext: bytes(&encryption["ct"]).into(),
    };
    let info = bytes(&vector["info"]);
    let aad = bytes(&encryption["aad"]);
    let plaintext = bytes(&encryption["pt"]);
    assert_eq!(
        crypto.hpke_open(config(), &ciphertext, &pair.private[..], &info, &aad).unwrap(),
        plaintext
    );
    for export in vector["exports"].as_array().unwrap() {
        let secret = crypto
            .hpke_setup_receiver_and_export(
                config(),
                &bytes(&vector["enc"]),
                &pair.private[..],
                &info,
                &bytes(&export["exporter_context"]),
                32,
            )
            .unwrap();
        assert_eq!(&secret[..], bytes(&export["exported_value"]));
    }
    assert!(
        crypto.hpke_open(config(), &ciphertext, &pair.private[..], &info, b"wrong aad").is_err()
    );
    let mut corrupted = bytes(&encryption["ct"]);
    corrupted[0] ^= 1;
    ciphertext.ciphertext = corrupted.into();
    assert!(crypto.hpke_open(config(), &ciphertext, &pair.private[..], &info, &aad).is_err());
    let sealed = crypto.hpke_seal(config(), &pair.public, &info, &aad, &plaintext).unwrap();
    assert_eq!(
        crypto.hpke_open(config(), &sealed, &pair.private[..], &info, &aad).unwrap(),
        plaintext
    );
}

#[test]
fn migrated_apis_reject_invalid_lengths_and_generate_fresh_keys() {
    let crypto = RustCrypto::default();
    for alg in [AeadType::Aes128Gcm, AeadType::Aes256Gcm, AeadType::ChaCha20Poly1305] {
        let key = vec![0; alg.key_size()];
        for len in [0, 11, 13] {
            let nonce = vec![0; len];
            assert_eq!(
                crypto.aead_encrypt(alg, &key, b"data", &nonce, b"aad"),
                Err(CryptoError::InvalidLength)
            );
            assert_eq!(
                crypto.aead_decrypt(alg, &key, &[0; 32], &nonce, b"aad"),
                Err(CryptoError::InvalidLength)
            );
        }
    }
    let scheme = SignatureScheme::ECDSA_SECP256R1_SHA256;
    for len in [0, 31, 33] {
        let key = vec![1; len];
        assert_eq!(crypto.sign(scheme, b"data", &key), Err(CryptoError::InvalidLength));
        assert!(SignatureKeyPair::from_raw(scheme, key, vec![]).sign(b"data").is_err());
    }
    let first = davey::SigningKeyPair::generate();
    let second = davey::SigningKeyPair::generate();
    assert_ne!(first.private, second.private);
    let signed = crypto.sign(scheme, b"generated key", &first.private).unwrap();
    crypto.verify_signature(scheme, b"generated key", &first.public, &signed).unwrap();
}
