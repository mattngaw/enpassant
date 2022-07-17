# Keeping Track of Pieces

Inside our project directory, let's create a new file in `src`, we can call it `board.rs`.

```text
> touch src/board.rs
```

Great start! Now what?

> **FRIEND:** How you would go about representing all the pieces on a chess board?

Well, a chessboard is an 8x8 grid, and each square can hold at most one piece. It kinda sounds like 2D array.

> **FRIEND:** Something like this?

```rust,noplayground
pub struct Board {
    board: [[SomethingGoesHere; 8]; 8]
}
```

Yeah! Except, we want to make an array of pieces, right? How about we assign a number to each piece? An empty square can be `0`, a pawn can be `1`, a knight can be—

> **FRIEND:** No no no! This is Rust, not C or Java. We can use enums!
<!-- TODO: Consider C and Java enums -->

```rust, noplayground
pub enum Color {
    White,
    Black
}

pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
```

> **FRIEND:** Here we strictly defined a set of variants for each enum. `Color` can only ever be `White` or `Black`, and `Role` can ever only be one of those pieces. On the other hand, those integers you wanted to use, they could be anything from `INT_MIN` to `INT_MAX`, or whatever the range was for the integer type you were using.

Ok, but you still haven't replaced the `SomethingGoesHere`. What do these enums provide us?

```rust,noplayground
pub struct Piece {
    color: Color,
    role: Role
}

pub struct Board {
    board: [[Option<Piece>; 8]; 8]
}
```

Whoa, what did you just do?

> **FRIEND:** Using our new enums, we can created a `Piece` struct. But of course, a chess board isn't entirely made up of pieces—there are empty squares. So we made the board a 2D  array of `Option<Piece>`.

Wait, I've seen `Option` before. Isn't it also an `enum`? It has two variants, `Some` and `None`.

> **FRIEND:** Right. If an index in our array doesn't contain a piece, i.e. it's an empty square, it will contain `None`. But if it has a piece, at the index there will be `Some(p)`, where `p` is a `Piece`.

I see! This is much more elegant that what I was going to do.

Now that we know how we're storing our pieces, we can write some methods for our board, right?

> **FRIEND:** Not quite. It would be more idiomatic if we represented the ranks (rows) and files (columns) of a chess board with enums as well.

```rust,noplayground
pub enum Rank {
    First = 0usize,
    Second = 1usize,
    Third = 2usize,
    Fourth = 3usize,
    Fifth = 4usize,
    Sixth = 5usize,
    Seventh = 6usize
    Eighth = 7usize
}

pub enum File {
    A = 0usize,
    B = 1usize,
    C = 2usize,
    D = 3usize,
    E = 4usize,
    F = 5usize,
    G = 6usize,
    H = 7usize
}
```

Okay... why did you assign values to the variants? And why do they have `usize` after them?

## CONTINUE FROM SomethingGoesHere

```rust,noplayground
impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; 8]]
        }
    }
}
```
