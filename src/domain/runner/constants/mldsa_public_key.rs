#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MLDSAPublicKeyMetadata {
    MLDSA44 = 1312,
    MLDSA65 = 1952,
    MLDSA87 = 2592,
}

impl MLDSAPublicKeyMetadata {
    pub const fn from_level(level: u8) -> Option<Self> {
        match level {
            0 => Some(MLDSAPublicKeyMetadata::MLDSA44),
            1 => Some(MLDSAPublicKeyMetadata::MLDSA65),
            2 => Some(MLDSAPublicKeyMetadata::MLDSA87),
            _ => None,
        }
    }

    pub const fn from_bytes_len(len: usize) -> Option<Self> {
        match len {
            1312 => Some(MLDSAPublicKeyMetadata::MLDSA44),
            1952 => Some(MLDSAPublicKeyMetadata::MLDSA65),
            2592 => Some(MLDSAPublicKeyMetadata::MLDSA87),
            _ => None,
        }
    }

    pub const fn to_level(self) -> u8 {
        match self {
            MLDSAPublicKeyMetadata::MLDSA44 => 0,
            MLDSAPublicKeyMetadata::MLDSA65 => 1,
            MLDSAPublicKeyMetadata::MLDSA87 => 2,
        }
    }

    pub const fn security_level(self) -> u8 {
        match self {
            MLDSAPublicKeyMetadata::MLDSA44 => 2,
            MLDSAPublicKeyMetadata::MLDSA65 => 3,
            MLDSAPublicKeyMetadata::MLDSA87 => 5,
        }
    }

    pub const fn private_key_len(self) -> usize {
        match self {
            MLDSAPublicKeyMetadata::MLDSA44 => 2560,
            MLDSAPublicKeyMetadata::MLDSA65 => 4032,
            MLDSAPublicKeyMetadata::MLDSA87 => 4896,
        }
    }

    pub const fn signature_len(self) -> usize {
        match self {
            MLDSAPublicKeyMetadata::MLDSA44 => 2420,
            MLDSAPublicKeyMetadata::MLDSA65 => 3309,
            MLDSAPublicKeyMetadata::MLDSA87 => 4627,
        }
    }

    pub const fn as_u64(self) -> u64 {
        self as u64
    }

    pub const fn as_usize(self) -> usize {
        self as usize
    }

    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    pub const fn name(self) -> &'static str {
        match self {
            MLDSAPublicKeyMetadata::MLDSA44 => "ML-DSA-44",
            MLDSAPublicKeyMetadata::MLDSA65 => "ML-DSA-65",
            MLDSAPublicKeyMetadata::MLDSA87 => "ML-DSA-87",
        }
    }
}

impl TryFrom<usize> for MLDSAPublicKeyMetadata {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::from_bytes_len(value).ok_or(())
    }
}

impl TryFrom<u32> for MLDSAPublicKeyMetadata {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_bytes_len(value as usize).ok_or(())
    }
}

impl From<MLDSAPublicKeyMetadata> for usize {
    fn from(val: MLDSAPublicKeyMetadata) -> Self {
        val.as_usize()
    }
}

impl From<MLDSAPublicKeyMetadata> for u32 {
    fn from(val: MLDSAPublicKeyMetadata) -> Self {
        val.as_u32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_level() {
        assert_eq!(
            MLDSAPublicKeyMetadata::from_level(0),
            Some(MLDSAPublicKeyMetadata::MLDSA44)
        );
        assert_eq!(
            MLDSAPublicKeyMetadata::from_level(1),
            Some(MLDSAPublicKeyMetadata::MLDSA65)
        );
        assert_eq!(
            MLDSAPublicKeyMetadata::from_level(2),
            Some(MLDSAPublicKeyMetadata::MLDSA87)
        );
        assert_eq!(MLDSAPublicKeyMetadata::from_level(3), None);
    }

    #[test]
    fn test_from_bytes_len() {
        assert_eq!(
            MLDSAPublicKeyMetadata::from_bytes_len(1312),
            Some(MLDSAPublicKeyMetadata::MLDSA44)
        );
        assert_eq!(
            MLDSAPublicKeyMetadata::from_bytes_len(1952),
            Some(MLDSAPublicKeyMetadata::MLDSA65)
        );
        assert_eq!(
            MLDSAPublicKeyMetadata::from_bytes_len(2592),
            Some(MLDSAPublicKeyMetadata::MLDSA87)
        );
        assert_eq!(MLDSAPublicKeyMetadata::from_bytes_len(1000), None);
    }

    #[test]
    fn test_associated_lengths() {
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA44.private_key_len(), 2560);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA65.private_key_len(), 4032);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA87.private_key_len(), 4896);

        assert_eq!(MLDSAPublicKeyMetadata::MLDSA44.signature_len(), 2420);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA65.signature_len(), 3309);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA87.signature_len(), 4627);
    }

    #[test]
    fn test_security_levels() {
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA44.security_level(), 2);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA65.security_level(), 3);
        assert_eq!(MLDSAPublicKeyMetadata::MLDSA87.security_level(), 5);
    }
}
