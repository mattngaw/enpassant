# Keeping Track of Pieces I

Inside our project directory, let's create a new file in `src`, we can call it `board.rs`.

```text
> touch src/board.rs
```

Ok, now what?

> **FRIEND:** How you would go about representing all the pieces on a chess board?

Well, a chessboard is an 8x8 grid, and each square can hold at most one piece. It kinda sounds like 2D array. Something like this?

```rust,noplayground
pub struct Board {
    board: [[i32; 8]; 8]
}
```

We can assign a 32-bit integer (`i32`) to each piece? An empty square can be `0`, a pawn can be `1`, a knight can be—

> **FRIEND:** No no no! This is Rust, not C or Java. We can use enums.
<!-- TODO: Consider C and Java enums -->

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

> **FRIEND:** Here we strictly defined a set of variants for each enum. `Color` can only ever be `White` or `Black`, and `Role` can ever only be one of those pieces. On the other hand, using `i32` meant a piece on our board could be anything from `-2147483648` to `2147483647`.

Ok, but you still haven't replaced the `SomethingGoesHere`. What do these enums buy us?

```rust,noplayground
pub struct Piece {
    color: Color,
    role: Role,
}

pub struct Board {
    board: [[Option<Piece>; 8]; 8]
}
```

Whoa, what did you just do?

> **FRIEND:** Using our new enums, I created a `Piece` struct, which is now the combination of a `Color` and a `Role`. But of course, a chessboard isn't entirely made up of pieces—there are empty squares. So I made the board a 2D  array of `Option<Piece>`.

Wait, I've seen `Option` before. Isn't it also an `enum`? It has two variants, `Some` and `None`.

> **FRIEND:** Right. If an index in our array doesn't contain a piece, i.e. it's an empty square, it will hold `None`. But if it has a piece, at the index there will be `Some(p)`, where `p` is a `Piece`. Alternatively, we could've done this...

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

I see. This is much more elegant that what I was going to do.

What about using enums to define the ranks (rows) and files (columns) on the chessboard?

> **FRIEND:** Great idea.

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

And we assign values to our enum variants like we might in C to use these enums as indices into our `Board`?

> **Friend:** Yep. You're catching on.

Great! Now we can start writing methods for our new `Board` struct right?

> **Friend:** Not quite. There's a concept in chess programming that I want to show you first.
