use candid::Principal;


pub struct SubAccount {
    pub bytes: [u8; 32],
}

impl SubAccount {

    pub fn derive_subaccount(principal: &Principal) -> Self {
        let principal_bytes = principal.as_slice();
        let mut subaccount = [0u8; 32];
        let start_index = subaccount.len() - principal_bytes.len();
    
        for (i, &byte) in principal_bytes.iter().enumerate() {
            subaccount[start_index + i] = byte;
        }
    
        Self { bytes: subaccount }
    }

    /// Create a `SubAccount` from a byte array.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let bytes = <[u8; 32]>::try_from(bytes).map_err(|_| "Invalid byte length for SubAccount".to_string())?;
        Ok(Self { bytes })
    }

    /// Create a `SubAccount` from a `Principal`.
    pub fn from_principal(principal: Principal) -> Self {
        let mut bytes = [0u8; 32];
        let principal_bytes = principal.as_slice();
        let start_index = bytes.len() - principal_bytes.len();

        bytes[start_index..].copy_from_slice(principal_bytes);
        Self { bytes }
    }

    /// Create a `SubAccount` from an ID.
    pub fn from_id(id: u8) -> Self {
        let mut bytes = [0u8; 32];
        bytes[31] = id; // Place the ID in the last byte
        Self { bytes }
    }

    /// Convert `SubAccount` to a `Uint8Array` equivalent.
    pub fn to_uint8_array(&self) -> [u8; 32] {
        self.bytes
    }
}
