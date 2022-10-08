use {curve25519_dalek::{
    scalar::Scalar,
    constants::RISTRETTO_BASEPOINT_TABLE,
    ristretto::RistrettoPoint,
    ristretto::CompressedRistretto,
},
    std::convert::TryFrom,
serde::{Deserialize, Serialize},
    solana_sdk::{
        instruction::Instruction,
        message::Message,
        pubkey::Pubkey,
        signature::Signature,
        signer::{Signer, SignerError},
    },
    std::convert::TryInto,
    zeroize::Zeroize,
};

#[cfg(not(target_os = "solana"))]
use {
    rand::rngs::OsRng,
    rand::{CryptoRng, RngCore},
    std::{
        fmt,
        fs::{self, File, OpenOptions},
        io::{Read, Write},
        path::Path,
    },
};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Zeroize)]
pub struct PublicKey(pub(crate) RistrettoPoint);

pub struct SharedSecret(pub(crate) RistrettoPoint);


#[derive(Zeroize)]
#[zeroize(drop)]
pub struct EphemeralSecret(pub(crate) Scalar);

impl EphemeralSecret{
    pub fn new<T: CryptoRng + RngCore>(mut csprng: T) -> Self{
        Self(Scalar::random(&mut csprng))
    }
    pub fn diffie_hellman(&self, other: &PublicKey) -> SharedSecret{
        SharedSecret(self.0 * other.0)
    }
}

impl<'a> From<&'a EphemeralSecret> for PublicKey {
    fn from(secret: &'a EphemeralSecret) -> PublicKey {
        Self(&secret.0 * &RISTRETTO_BASEPOINT_TABLE)
    }
}