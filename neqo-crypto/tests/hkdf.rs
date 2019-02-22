use neqo_crypto::constants::*;
use neqo_crypto::hkdf;
use neqo_crypto::{init_db, SymKey};

const SALT: &[u8] = &[
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
];

const IKM: &[u8] = &[
    0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
];

fn import_keys(cipher: Cipher) -> (SymKey, SymKey) {
    let l = match cipher {
        TLS_AES_128_GCM_SHA256 | TLS_CHACHA20_POLY1305_SHA256 => 32,
        TLS_AES_256_GCM_SHA384 => 48,
        _ => unreachable!(),
    };
    (
        SymKey::import(cipher, &SALT[0..l]).expect("import salt"),
        SymKey::import(cipher, &IKM[0..l]).expect("import IKM"),
    )
}

#[test]
fn extract_sha256() {
    init_db("./db");
    let expected = &[
        0xa5, 0x68, 0x02, 0x5a, 0x95, 0xc9, 0x7f, 0x55, 0x38, 0xbc, 0xf7, 0x97, 0xcc, 0x0f, 0xd5,
        0xf6, 0xa8, 0x8d, 0x15, 0xbc, 0x0e, 0x85, 0x74, 0x70, 0x3c, 0xa3, 0x65, 0xbd, 0x76, 0xcf,
        0x9f, 0xd3,
    ];
    let (salt, ikm) = import_keys(TLS_AES_128_GCM_SHA256);
    let prk = hkdf::extract(TLS_VERSION_1_3, TLS_AES_128_GCM_SHA256, &salt, &ikm)
        .expect("HKDF Extract should work");
    let raw_prk = prk.as_bytes().expect("key should have bytes");
    assert_eq!(raw_prk, expected);
}

#[test]
fn derive_secret() {
    init_db("./db");
    // let _a = Aead::new();
}
