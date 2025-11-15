#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsensusFlags(u64);

impl ConsensusFlags {
    pub const NONE: Self = Self(0b00000000);
    pub const UNSAFE_QUANTUM_SIGNATURES_ALLOWED: Self = Self(0b00000001);
    pub const RESERVED_FLAG_1: Self = Self(0b00000010);
    pub const RESERVED_FLAG_2: Self = Self(0b00000100);

    pub const fn new() -> Self {
        Self::NONE
    }

    pub const fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn intersects(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }

    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.0;
    }

    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
}

impl std::ops::BitOr for ConsensusFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ConsensusFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for ConsensusFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for ConsensusFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitXor for ConsensusFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for ConsensusFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl std::ops::Sub for ConsensusFlags {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl std::ops::SubAssign for ConsensusFlags {
    fn sub_assign(&mut self, rhs: Self) {
        self.remove(rhs);
    }
}

impl std::ops::Not for ConsensusFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl std::fmt::Binary for ConsensusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.0, f)
    }
}

impl From<u64> for ConsensusFlags {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<ConsensusFlags> for u64 {
    fn from(flags: ConsensusFlags) -> Self {
        flags.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_flags() {
        let mut flags =
            ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED | ConsensusFlags::RESERVED_FLAG_1;

        assert!(flags.contains(ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED));
        assert!(flags.contains(ConsensusFlags::RESERVED_FLAG_1));
        assert!(!flags.contains(ConsensusFlags::RESERVED_FLAG_2));

        flags.insert(ConsensusFlags::RESERVED_FLAG_2);
        assert!(flags.contains(ConsensusFlags::RESERVED_FLAG_2));

        flags.remove(ConsensusFlags::RESERVED_FLAG_1);
        assert!(!flags.contains(ConsensusFlags::RESERVED_FLAG_1));

        let combined = ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED
            | ConsensusFlags::RESERVED_FLAG_1
            | ConsensusFlags::RESERVED_FLAG_2;

        assert_eq!(combined.as_u64(), 0b00000111);
    }

    #[test]
    fn test_from_u64() {
        let flags = ConsensusFlags::from_u64(0b00000101);
        assert!(flags.contains(ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED));
        assert!(flags.contains(ConsensusFlags::RESERVED_FLAG_2));
    }

    #[test]
    fn test_to_be_bytes() {
        let flags = ConsensusFlags::UNSAFE_QUANTUM_SIGNATURES_ALLOWED;
        let bytes = flags.to_be_bytes();
        assert_eq!(bytes, [0, 0, 0, 0, 0, 0, 0, 1]);
    }
}
