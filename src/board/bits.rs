use std::ops::{BitOr, BitAnd, BitXor, Not};

use super::{Rank, File};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Square(u8);

impl Square {
    pub const COUNT: usize = 64;

    pub fn new(value: u8) -> Self {
        assert!(value < 64, "value must be between 0 and 64");
        Square::new_unchecked(value)
    }

    pub fn new_unchecked(value: u8) -> Self {
        Square(value)
    }

    pub fn from_file_and_rank(f: File, r: Rank) -> Self {
        let file_index = f as u8;
        let rank_index = r as u8;
        Square(rank_index * 8 + file_index)
    }

    #[inline]
    pub fn from_singular_bitboard(b: Bitboard) -> Self {
        assert!(b.is_singular());
        Square(b.0.trailing_zeros() as u8)
    }

    #[inline]
    pub fn file(self) -> File {
        File::new(self.0 % 8)
    }

    #[inline]
    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 8)
    }

    #[inline]
    pub fn coords(self) -> (File, Rank) {
        (self.file(), self.rank())
    }

    #[inline]
    pub fn flipped(self) -> Self {
        Square(63 - self.0)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Bitboard(u64);

impl Bitboard {
    const EMPTY_MASK: u64 = 0x0000_0000_0000_0000;

    const FULL_MASK: u64 = 0xFFFF_FFFF_FFFF_FFFF;

    const FILE_MASKS: [u64; 8] = [
        0x8080_8080_8080_8080,
        0x4040_4040_4040_4040,
        0x2020_2020_2020_2020,
        0x1010_1010_1010_1010,
        0x0808_0808_0808_0808,
        0x0404_0404_0404_0404,
        0x0202_0202_0202_0202,
        0x0101_0101_0101_0101,
    ];

    const RANK_MASKS: [u64; 8] = [
        0xFF00_0000_0000_0000,
        0x00FF_0000_0000_0000,
        0x0000_FF00_0000_0000,
        0x0000_00FF_0000_0000,
        0x0000_0000_FF00_0000,
        0x0000_0000_00FF_0000,
        0x0000_0000_0000_FF00,
        0x0000_0000_0000_00FF,
    ];

    const SINGULAR_MASKS: [u64; 64] = {
        let mut masks: [u64; 64] = [1u64; 64];
        let mut i: usize = 0;
        while i < 64 {
            masks[i] = 1u64 << i;
            i += 1;
        }
        masks

    };

    pub fn new(value: u64) -> Self {
        Bitboard(value)
    }

    #[inline]
    pub fn from_square(s: Square) -> Self {
        debug_assert!(s.0 < 64);
        Bitboard(Self::SINGULAR_MASKS[s.0 as usize])
    }

    #[inline]
    pub fn from_rank(r: Rank) -> Self {
        Bitboard(Self::RANK_MASKS[r as usize])
    }

    #[inline]
    pub fn from_file(f: File) -> Self {
        Bitboard(Self::FILE_MASKS[f as usize])
    }

    #[inline]
    pub fn empty() -> Self {
        Bitboard(Self::EMPTY_MASK)
    }

    #[inline]
    pub fn full() -> Self {
        Bitboard(Self::FULL_MASK)
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    #[inline]
    pub fn is_any(self) -> bool {
        self.0 != 0u64
    }

    #[inline]
    pub fn is_singular(self) -> bool {
        self.0.is_power_of_two()
    }

    #[inline]
    pub fn flipped(self) -> Self {
        Bitboard(self.0.reverse_bits())
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}