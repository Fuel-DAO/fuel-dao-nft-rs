use candid::Principal;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;

use super::subaccount::SubAccount;

pub struct AccountIdentifier {
    bytes: [u8; 32],
}

impl AccountIdentifier {
    /// Create an `AccountIdentifier` from a hexadecimal string.
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let decoded = hex::decode(hex).map_err(|e| e.to_string())?;
        let bytes = <[u8; 32]>::try_from(decoded.as_slice())
            .map_err(|_| "Invalid byte length for AccountIdentifier".to_string())?;
        Ok(Self { bytes })
    }

    /// Create an `AccountIdentifier` from a `Principal` and optional `SubAccount`.
    pub fn from_principal(principal: Principal, sub_account: Option<SubAccount>) -> Self {
        let mut hash_input = Vec::new();
        hash_input.extend_from_slice(principal.as_slice());
        hash_input.extend_from_slice(
            sub_account
                .map_or([0; 32], |sub| sub.to_uint8_array())
                .as_ref(),
        );

        let hash = Sha256::digest(&hash_input);
        let bytes = <[u8; 32]>::try_from(hash.as_slice()).expect("Hash must be 32 bytes");

        Self { bytes }
    }

    /// Convert `AccountIdentifier` to a hexadecimal string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    /// Convert `AccountIdentifier` to a `Uint8Array` equivalent.
    pub fn to_uint8_array(&self) -> [u8; 32] {
        self.bytes
    }

    /// Convert `AccountIdentifier` to a vector of numbers.
    pub fn to_numbers(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Generate a hash representation of the `AccountIdentifier`.
    pub fn to_account_identifier_hash(&self) -> [u8; 32] {
        self.bytes
    }
}
