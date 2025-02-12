use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, All};
use secp256k1::rand::rngs::OsRng;

pub struct Wallet {
    secp: Secp256k1<All>,
    secret_key: SecretKey,
    pub public_key: PublicKey,
    pub msg: Message,
}

impl Wallet {
    pub fn new() -> Wallet {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        Wallet {
            secp,
            secret_key,
            public_key,
            msg: Wallet::create_empty_message(),
        }
    }

    fn create_empty_message() -> Message {
        let empty_hash = [0u8; 32]; // 全零的 32 字节数组
        Message::from_digest(empty_hash)
    }

    pub fn sign(&self, hash: String) -> secp256k1::ecdsa::Signature
    {
        let decoded_bytes = hex::decode(&hash).expect("Invalid hex string");
        let message = Message::from_digest(decoded_bytes.try_into().expect("Invalid length"));
        self.secp.sign_ecdsa(&message, &self.secret_key)
    }

    pub fn verify(&self, msg: Message, signature: secp256k1::ecdsa::Signature, public_key: PublicKey) -> bool
    {
        let result = self.secp.verify_ecdsa(&msg, &signature, &public_key);
        match result {
            Ok(_) => true,
            Err(_) => false,  
        }
    }
}