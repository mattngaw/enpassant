//! Structs for working with and keeping track of squares.
//!
//! 

use std::ops::{BitOr, BitAnd, BitXor, Not};

use crate::board::{Rank, File, Flippable};

/// The monad of the chessboard upon which chess pieces exist.
///
/// Squares index into a chessboard via rank-major order.
/// ```text
/// From white's perspective            From black's perspective
///
/// 8   56 57 58 59 60 61 62 63         1    7  6  5  4  3  2  1  0
/// 7   48 49 50 51 52 53 54 55         2   15 14 13 12 11 10  9  8
/// 6   40 41 42 43 44 45 46 47         3   23 22 21 20 19 18 17 16
/// 5   32 33 34 35 36 37 38 39         4   31 30 29 28 27 26 25 24
/// 4   24 25 26 27 28 29 30 31         5   39 38 37 36 35 34 33 32
/// 3   16 17 18 19 20 21 22 23         6   47 46 45 44 43 42 41 40
/// 2    8  9 10 11 12 13 14 15         7   55 54 53 52 51 50 49 48
/// 1    0  1  2  3  4  5  6  7         8   63 62 61 60 59 58 57 56
/// 
///      a  b  c  d  e  f  g  h              a  b  c  d  e  f  g  h
/// ```
///
/// [`Squares`](Square) are
/// [*color-agnostic*](https://www.chessprogramming.org/Color_Flipping#Monochrome),
/// e.g. viewing the board from white's perspective means index 42 is c6, but
/// from black's perspective index 42 is f3.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
pub struct Square(u8);

impl Square {
    /// Number of squares on chessboard.
    pub const COUNT: usize = 64;

    /// Constructs a [`Square`] from a [`u8`].
    ///
    /// # Panics
    /// Panics if `value > 63`.
    ///
    /// # Postcondition
    /// The returned square represents a valid square on a chessboard.
    pub fn new(value: u8) -> Self {
        assert!(value < 64, "value must be between 0 and 64");
        Square::new_unchecked(value)
    }

    /// Constructs a [`Square`] from a [`u8`].
    ///
    /// The caller of this method must know what they're doing.
    ///
    /// # Precondition
    /// `0 <= value <= 63`
    pub fn new_unchecked(value: u8) -> Self {
        Square(value)
    }

    /// Constructs a [`Square`] from a [`File`] and [`Rank`].
    #[inline]
    pub fn from_coords(f: File, r: Rank) -> Self {
        let file_index = f as u8;
        let rank_index = r as u8;
        Square(rank_index * 8 + file_index)
    }

    /// Gets the [`File`] of a [`Square`].
    #[inline]
    pub fn file(self) -> File {
        File::new(self.0 % 8)
    }
    
    /// Gets the [`Rank`] of a [`Square`].
    #[inline]
    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 8)
    }

    /// Returns a tuple of the [`File`] and [`Rank`].
    #[inline]
    pub fn coords(self) -> (File, Rank) {
        (self.file(), self.rank())
    }

    /// Returns the index (`i`, the inner value of the square).
    /// 
    /// # Postcondition
    /// `0 <= i <= 63`
    #[inline]
    pub fn index(&self) -> usize {
        let i = self.index_unchecked();
        assert!(i < 64, "invalid square index");
        i
    }

    /// Returns the index (could be invalid index).
    #[inline]
    pub fn index_unchecked(&self) -> usize {
        self.0 as usize
    }
}

impl TryFrom<Bitboard> for Square {
    type Error = &'static str;

    /// Constructs a [`Square`] from a bitboard.
    ///
    /// Returns
    #[inline]
    fn try_from(b: Bitboard) -> Result<Self, Self::Error> {
        if b.is_singular() { Ok( Square(b.0.trailing_zeros() as u8) ) }
        else { Err("Square can only be made from singleton bitboard") }
    }
}

impl Flippable for Square {
    #[inline]
    fn flipped(&self) -> Self {
        Square(63 - self.0)
    }
}

/// A set of occupied squares.
/// 
/// A bitboard keeps track of which squares are occupied by some piece. They 
/// can also be used to describe possible moves or attacks for a piece. For 
/// example, some bitboards from the starting position of a chess game:
///
/// ```text
/// 8  . x . . . . x .         8  . . . . . . . .         8  . . . . . . . .
/// 7  . . . . . . . .         7  . . . . . . . .         7  . . . . . . . .
/// 6  . . . . . . . .         6  . . . . . . . .         6  . . . . . . . .
/// 5  . . . . . . . .         5  . . . . . . . .         5  . . . . . . . .
/// 4  . . . . . . . .         4  . . . . . . . .         4  . . . . . . . .
/// 3  . . . . . . . .         3  . . . . . . . .         3  . . . . . . . .
/// 2  . . . . . . . .         2  x x x x x x x x         6  . . . . . . . .
/// 1  . x . . . . x .         1  x x x x x x x x         1  . x . . . . x .
/// 
///    a b c d e f g h            a b c d e f g h            a b c d e f g h
/// 
/// bitboard of knights        bitboard of white pieces   bitboard of white knights
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug)]
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

    /// Constructs a new bitboard given a 64-bit integer.
    pub fn new(value: u64) -> Self {
        Bitboard(value)
    }

    /// Constructs an empty bitboard.
    #[inline]
    pub fn empty() -> Self {
        Bitboard(Self::EMPTY_MASK)
    }

    /// Constructs a full bitboard.
    #[inline]
    pub fn full() -> Self {
        Bitboard(Self::FULL_MASK)
    }

    /// Checks whether a bitboard is empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    /// Checks whether a bitboard is non-empty.
    #[inline]
    pub fn is_any(self) -> bool {
        self.0 != 0u64
    }

    /// Checks whether a bitboard is singular.
    #[inline]
    pub fn is_singular(self) -> bool {
        self.0.is_power_of_two()
    }

    /// Returns the number of occupied squares in the bitboard.
    #[inline]
    pub fn count(self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns the occupied square, if any, closest to index 63.
    #[inline]
    pub fn largest_square(self) -> Option<Square> {
        self.is_any().then_some({
            let value = self.0.trailing_zeros() as u8; 
            Square::new_unchecked(value)
        })
    }

    /// Returns the occupied square, if any, closest to index 0.
    pub fn smallest_square(self) -> Option<Square> {
        self.is_any().then_some({
            let value = 63 - self.0.leading_zeros() as u8; 
            Square::new_unchecked(value)
        })
    }

    /// Returns whether the square is occupied.
    #[inline]
    pub fn contains(self, s: Square) -> bool {
        (Bitboard::from(s) & self).is_any()
    }

    /// Adds a square to the bitboard.
    /// 
    /// If the square isn't in the set, `true` is returned.
    /// If the square is in the set, `false` is returned.
    pub fn add(&mut self, s: Square) -> bool {
        match self.contains(s) {
            true => false,
            false => {
                self.0 |= 1 << s.0;
                true
            }
        }
    }

    /// Removes a square from the bitboard.
    /// 
    /// If the square is in the set, `true` is returned.
    /// If the square isn't in the set `false` is returned.
    pub fn remove(&mut self, s: Square) -> bool {
        match self.contains(s) {
            true => {
                self.0 &= !(1 << s.0);
                true
            },
            false => false
        }
    }
}

impl From<Square> for Bitboard {
    /// Constructs a bitboard from a [`Square`].
    ///
    /// # Postcondition
    /// The returned bitboard is singular
    #[inline]
    fn from(s: Square) -> Self {
        debug_assert!(s.0 < 64);
        Bitboard(Self::SINGULAR_MASKS[s.index()])
    }
}

impl From<File> for Bitboard {
    /// Constructs a bitboard from a [`File`].
    /// # Postcondition
    /// The returned bitboard represents the squares in `f`.
    #[inline]
    fn from(f: File) -> Self {
        Bitboard(Self::FILE_MASKS[f as usize])
    }
}

impl From<Rank> for Bitboard {
    /// Constructs a bitboard from a [`Rank`].
    /// # Postcondition
    /// The returned bitboard represents the squares in `r`.
    #[inline]
    fn from(r: Rank) -> Self {
        Bitboard(Self::RANK_MASKS[r as usize])
    }
}

impl Flippable for Bitboard {
    #[inline]
    fn flipped(&self) -> Self {
        Bitboard(self.0.reverse_bits())
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    /// Union of two bitboards.
    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    /// Intersection of two bitboards.
    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    /// Disjoin union of two bitboards.
    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    /// Complement of a bitboard.
    #[inline]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}
