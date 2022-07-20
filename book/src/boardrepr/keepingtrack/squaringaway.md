# Squaring Away the Code

> **FRIEND:** Let's add to our `Board` struct.

```rust, noplayground
pub struct Board {
    colors: [Bitboard; 2],
    roles: [Bitboard; 5],
    mailbox: Mailbox
};
```

> **FRIEND:** Our colors and roles bitboards are stored in arrays, and we can use our enums to index into them. Our old `Option<Piece>` array can be encapsulated into a `Mailbox` struct that we'll soon define.

```rust, noplayground
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Bitboard(u64);

impl Bitboard { ... }
```

> **FRIEND:** Our bitboard is just represented by a 64-bit number, and we don't care about the sign of our board, so we'll use a `u64`. For simplicity's sake, we're using a tuple struct, where we can access the field by index (`b.0`, where `b` is a `Bitboard`).

```rust, noplayground
impl Bitboard {
    pub fn new(value: u64) -> Self {
        Bitboard(value)
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    pub fn is_any(self) -> bool {
        self.0 != 0u64
    }

    ...
}
```

> **FRIEND:** We'll also add some useful constants (called masks) for specific bitboards that are useful to have in future functions.

```rust, noplayground
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

    pub fn new(value: u64) -> Self { ... }

    ...

    pub fn from_square( ? ) -> Self {
        todo!()
    }

    pub fn from_rank(r: Rank) -> Self {
        Bitboard(Self::RANK_MASKS[r as usize])
    }

    pub fn from_file(f: File) -> Self {
        Bitboard(Self::FILE_MASKS[f as usize])
    }

    pub fn empty() -> Self {
        Bitboard(Self::EMPTY_MASK)
    }

    pub fn full() -> Self {
        Bitboard(Self::FULL_MASK)
    }
}
```

> **FRIEND:** Most of these masks are self-explanatory. `FILE_MASKS` and `RANK_MASKS` are indexable by row and file. `SINGULAR_MASKS` is indexable by square, but notice that it is constructed within a block, in which computation is done to create an array of masks that correspond with their respective index.

As opposed to writing out 64 lines of masks like we did for rank and file?

> **FRIEND:** Exactly.
>
> **FRIEND:** You also might have noticed that I've left the `from_square` method blank. We'll get back to it soon, since it's dependent on how we implement squares.
>
> **FRIEND:** We're also going to implement some traits[^1] for our `Bitboard` struct to make it easier to do bitwise operations.

```rust, noplayground
use std::ops::{BitOr, BitAnd, BitXor, Not};

...

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}
```

Sweet. So now we can perform operations like these.

```rust, noplayground
let b1 = Bitboard::from_file(File::G);
let b2 = Bitboard::from_rank(Rank::Fifth);
```

```text
                8  . . . . . . x . 
                7  . . . . . . x . 
                6  . . . . . . x . 
                5  x x x x x x x x 
b1 | b2    =    4  . . . . . . x . 
                3  . . . . . . x . 
                2  . . . . . . x . 
                1  . . . . . . x . 
        
                   a b c d e f g h


                8  . . . . . . . . 
                7  . . . . . . . . 
                6  . . . . . . . . 
                5  . . . . . . x . 
b1 & b2    =    4  . . . . . . . . 
                3  . . . . . . . . 
                2  . . . . . . . . 
                1  . . . . . . . . 
        
                   a b c d e f g h
```

> **FRIEND:** Yep.
> 
> **FRIEND:** Now we need to go back and implement our `from_square` method.

Most chess engines and libraries just type alias a `Square` to an integer type.

> **FRIEND:** That could work, but it might be safer and cleaner if we wrap that integer in a struct. Doing this would allow us to write methods that also guarantee that we only have valid squares in our program, i.e. squares whose index on the board only range from 0 to 63 (squares with indices 64 and above don't make much sense).

```rust, noplayground
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

    pub fn from_singular_bitboard(b: Bitboard) -> Self {
        assert!(b.is_singular());
        Square(b.0.trailing_zeros() as u8)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }

    pub fn file(self) -> File {
        File::new(self.0 % 8)
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 8)
    }

    pub fn coords(self) -> (File, Rank) {
        (self.file(), self.rank())
    }

    pub fn flipped(self) -> Self {
        Square(63 - self.0)
    }
}

```

[^1]: [Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html)
