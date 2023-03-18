use super::QrCode;

use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCryptError};

pub struct Encrypted {
    pub encryption_key: String
}
pub struct NotEncrypted;

pub trait Encryption {
    fn cypher(&self, qrcode: &QrCode) -> Vec<u8>;
    fn decypher(&self, data: &Vec<u8>) -> Result<Vec<u8>, MagicCryptError>;
}

impl Encryption for Encrypted {
    fn cypher(&self, qrcode: &QrCode) -> Vec<u8> {
        new_magic_crypt!(&self.encryption_key, 256).encrypt_bytes_to_bytes(&qrcode.data)
    }

    fn decypher(&self, data: &Vec<u8>) -> Result<Vec<u8>, MagicCryptError> {
        new_magic_crypt!(&self.encryption_key, 256).decrypt_bytes_to_bytes(data)
    }
}

impl Encryption for NotEncrypted {
    fn cypher(&self, qrcode: &QrCode) -> Vec<u8> {
        qrcode.data.to_owned()
    }

    fn decypher(&self, data: &Vec<u8>) -> Result<Vec<u8>, MagicCryptError> {
        Ok(data.to_owned())
    }
}