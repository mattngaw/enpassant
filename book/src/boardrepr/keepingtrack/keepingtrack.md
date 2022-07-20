# Keeping Track of Pieces

Inside our project directory, let's create a new file in `src`. We can call it `board.rs`.

```text
> touch src/board.rs
```
How would you go about representing pieces on a chess board?

>**FRIEND:** Well, a chessboard is an 8x8 grid, and each square can hold at most one piece. It kinda sounds like 2D array. We should create a struct that contains everything regarding the state of a chessboard.

```rust,noplayground
pub struct Board {
    pieces: [[SomethingGoesHere; 8]; 8]
}
```

Typically we use an enum to distinguish between different types of pieces. Does Rust have something like that?

> **FRIEND:** Yep, check this out.

```rust, noplayground
pub enum Color {
    White,
    Black,
}

pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
```

> **FRIEND:** Here we strictly defined a set of variants for each enum. `Color` can only ever be `White` or `Black`, and `Role` can ever only be one of those pieces. We can then put this enums together to make a full piece.

```rust,noplayground
pub struct Piece {
    color: Color,
    role: Role,
}

pub struct Board {
    board: [[Option<Piece>; 8]; 8]
}
```

Whoa, what did you just put in our board array?

> **FRIEND:** Well, a chessboard isn't entirely made up of pieces—there are empty squares. So I made the board a 2D  array of `Option<Piece>`, which is also an enum of two variants: `Some` and `None`.  If an index in our array doesn't contain a piece, i.e. it's an empty square, it will hold `None`. But if it has a piece, at the index there will be `Some(p)`, where `p` is a `Piece`. Alternatively, we could've done this...

```rust,noplayground
pub enum Piece {
    Empty,
    Occupied(Color, Role),
}

pub struct Board {
    board: [[Piece; 8]; 8]
}
```

> **FRIEND:** ...but it's sort of just a worse version of `Option`, so we'll stick with the other definition.

Gotcha. Let's also define ranks and files on the chessboard.

```rust,noplayground
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6
    Eighth = 7,
}

pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
```

And we can assign values to our enum variants like we might in C to use these enums as indices into our `Board`?

> **Friend:** Sounds like a good idea. Now we can start writing functions for our new `Board` struct, right?

Not quite. There's another board representation concept in chess programming that I want to show you first.
