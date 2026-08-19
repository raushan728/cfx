use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::error::{CfxError, Result};

pub const MAGIC_BYTES: &[u8; 4] = b"CFX1";
pub const VERSION: u8 = 1;
pub const KDF_ID_ARGON2: u8 = 1;
pub const CIPHER_ID_CHACHA: u8 = 1;
pub const SALT_LEN: usize = 32;
pub const NONCE_LEN: usize = 8;

#[derive(Debug, Clone)]
pub struct CfxHeader {
    pub version: u8,
    pub kdf_id: u8,
    pub cipher_id: u8,
    pub salt: [u8; SALT_LEN],
    pub nonce: [u8; NONCE_LEN],
}

impl CfxHeader {
    pub fn new(salt: [u8; SALT_LEN], nonce: [u8; NONCE_LEN]) -> Self {
        Self {
            version: VERSION,
            kdf_id: KDF_ID_ARGON2,
            cipher_id: CIPHER_ID_CHACHA,
            salt,
            nonce,
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(MAGIC_BYTES)?;
        writer.write_u8(self.version)?;
        writer.write_u8(self.kdf_id)?;
        writer.write_u8(self.cipher_id)?;
        writer.write_all(&self.salt)?;
        writer.write_all(&self.nonce)?;
        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).map_err(|_| {
            CfxError::InvalidFormat("Failed to read magic bytes or file too short".into())
        })?;
        if &magic != MAGIC_BYTES {
            return Err(CfxError::InvalidFormat("Invalid magic bytes".into()));
        }

        let version = reader
            .read_u8()
            .map_err(|_| CfxError::InvalidFormat("Failed to read version".into()))?;
        if version != VERSION {
            return Err(CfxError::InvalidFormat(format!(
                "Unsupported version: {}",
                version
            )));
        }

        let kdf_id = reader
            .read_u8()
            .map_err(|_| CfxError::InvalidFormat("Failed to read KDF ID".into()))?;
        if kdf_id != KDF_ID_ARGON2 {
            return Err(CfxError::InvalidFormat(format!(
                "Unsupported KDF ID: {}",
                kdf_id
            )));
        }

        let cipher_id = reader
            .read_u8()
            .map_err(|_| CfxError::InvalidFormat("Failed to read Cipher ID".into()))?;
        if cipher_id != CIPHER_ID_CHACHA {
            return Err(CfxError::InvalidFormat(format!(
                "Unsupported Cipher ID: {}",
                cipher_id
            )));
        }

        let mut salt = [0u8; SALT_LEN];
        reader
            .read_exact(&mut salt)
            .map_err(|_| CfxError::InvalidFormat("Failed to read salt".into()))?;

        let mut nonce = [0u8; NONCE_LEN];
        reader
            .read_exact(&mut nonce)
            .map_err(|_| CfxError::InvalidFormat("Failed to read nonce".into()))?;

        Ok(Self {
            version,
            kdf_id,
            cipher_id,
            salt,
            nonce,
        })
    }
}
