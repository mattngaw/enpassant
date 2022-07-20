# Bits and Pieces

So far we're able to represent a chessboard like this.

```text
[[_, _, _, _, p, _, _, _],            8  . . . . . . . .
 [_, _, N, _, _, _, _, _],            7  . . . . . . . .
 [p, _, _, _, _, _, _, _],            6  . . . . . . . .
 [_, p, _, p, _, _, _, _],            5  . . . . . . . .
 [_, _, _, _, _, _, _, _],   ----->   4  . p . p . . . .
 [_, _, _, _, _, _, _, _],            3  p . . . . . . .
 [_, _, _, _, _, _, _, _],            2  . . N . . . . .
 [_, _, _, _, _, _, _, _]]            1  . . . . p . . .

                                         a b c d e f g h
```

> **FRIEND:** Wait, it's upside down?

Yep, because index 0 in our array maps to the first rank on a chessboard.

> **FRIEND:** Might have to stare at that for a while to understand it.

```text
[[a1, b1, c1, d1, e1, f1, g1, h1],
 [a2, b2, c2, d2, e2, f2, g2, h2],
 [a3, b3, c3, d3, e3, f3, g3, h3],
 [a4, b4, c4, d4, e4, f4, g4, h4],
 [a5, b5, c5, d5, e5, f5, g5, h5],
 [a6, b6, c6, d6, e6, f6, g6, h6],
 [a7, b7, c7, d7, e7, f7, g7, h7],
 [a8, b8, c8, d8, e8, f8, g8, h8]]
```

> **FRIEND:** Ah. That helps.

> **FRIEND:** You know, after some thought, this 2D array seems like a good way to check what's on a square. If I wanted to see what's on `c2`, then all I'd have to do is index using `Rank::Second` and `File::C`. But what if I wanted to ask, "Is there a knight?" Would I have to scan the entire 2D array?

You're catching on to one of the cons of this approach. All *square-centric* approaches are good at telling you what's on a square, but not where the pieces are (at least without scanning the entire board). This specific representation method is known as a *mailbox*.[^1]

The counterpart of square-centric is *piece-centric*. Piece-centric representations are good at answering, "Where are the pieces?" The most common method of piece-centric representation is the *bitboard*.[^2] Here are some examples from the starting position of a chess game.

```text
8  . x . . . . x .         8  . . . . . . . .         8  . . . . . . . .
7  . . . . . . . .         7  . . . . . . . .         7  . . . . . . . .
6  . . . . . . . .         6  . . . . . . . .         6  . . . . . . . .
5  . . . . . . . .         5  . . . . . . . .         5  . . . . . . . .
4  . . . . . . . .         4  . . . . . . . .         4  . . . . . . . .
3  . . . . . . . .         3  . . . . . . . .         3  . . . . . . . .
2  . . . . . . . .         2  x x x x x x x x         6  . . . . . . . .
1  . x . . . . x .         1  x x x x x x x x         1  . x . . . . x .

   a b c d e f g h            a b c d e f g h            a b c d e f g h

bitboard of knights        bitboard of white pieces   bitboard of white knights
```

> **FRIEND:** Whoa, these are a lot more specific than the mailbox we had earlier. Also, the pieces are symbolized by `x`'s now?

Yep, bitboards are essentially a binary string, a 64-bit integer, where the ones or `x`'s represent pieces that are present, and the zeroes or periods represent the absence of pieces.

> **FRIEND:** So, not an empty square.

Exactly. Notice that a zero in the bitboard of white's pieces symbolizes where there *aren't* white pieces, not that there aren't any pieces at all.

> **FRIEND:** I see what you're doing. If we had a bitboard for every piece, we would know where all the pieces on a chessboard are!

Pretty much. There are many different ways to use bitboards to keep track of all the information we need. In this book, we'll keep track of two color bitboards (all white pieces and all black pieces), and then five piece type (`Role`) bitboards (one for each piece, except for the king[^3]). Let's go back to the example bitboards. If you're familiar with binary/bitwise operations, you'll find bitboards intuitive.

```text
8  . x . . . . x .         8  . . . . . . . .         8  . . . . . . . .
7  . . . . . . . .         7  . . . . . . . .         7  . . . . . . . .
6  . . . . . . . .         6  . . . . . . . .         6  . . . . . . . .
5  . . . . . . . .         5  . . . . . . . .         5  . . . . . . . .
4  . . . . . . . .    &    4  . . . . . . . .    =    4  . . . . . . . .
3  . . . . . . . .   AND   3  . . . . . . . .         3  . . . . . . . .
2  . . . . . . . .         2  x x x x x x x x         6  . . . . . . . .
1  . x . . . . x .         1  x x x x x x x x         1  . x . . . . x .

   a b c d e f g h            a b c d e f g h            a b c d e f g h

knights                    white pieces               white knights


8  . . . . . . . .         8  x x x x x x x x         8  x x x x x x x x
7  . . . . . . . .         7  x x x x x x x x         7  x x x x x x x x
6  . . . . . . . .         6  . . . . . . . .         6  . . . . . . . .
5  . . . . . . . .         5  . . . . . . . .         5  . . . . . . . .
4  . . . . . . . .    |    4  . . . . . . . .    =    4  . . . . . . . .
3  . . . . . . . .   OR    3  . . . . . . . .         3  . . . . . . . .
2  x x x x x x x x         2  . . . . . . . .         6  x x x x x x x x
1  x x x x x x x x         1  . . . . . . . .         1  x x x x x x x x

   a b c d e f g h            a b c d e f g h            a b c d e f g h

white pieces               black pieces               all pieces


                           8  . . . . . . . .         8  x x x x x x x x
                           7  . . . . . . . .         7  x x x x x x x x
                           6  . . . . . . . .         6  x x x x x x x x
                           5  . . . . . . . .         5  x x x x x x x x
                      !    4  . . . . . . . .    =    4  x x x x x x x x
                     NOT   3  . . . . . . . .         3  x x x x x x x x
                           2  x x x x x x x x         6  . . . . . . . .
                           1  x x x x x x x x         1  . . . . . . . .

                              a b c d e f g h            a b c d e f g h

                           white pieces               everything but white pieces
```

Just using these operations, we can access all the information we could need about the pieces on a board. At an instruction level, this is also pretty cheap—bitwise operations typically have their own instruction.

To remain flexible, it's useful to have both piece-centric and square-centric representations of the board, even if they're redundant.

> **FRIEND:** Fine with me. Let's start writing this in Rust!

[^1]: ["Mailbox", Chess Programming Wiki](https://www.chessprogramming.org/Mailbox)

[^2]: ["Bitboards", Chess Programming Wiki](https://www.chessprogramming.org/Bitboards)

[^3]: Since there is only one king on each side, we need not keep an entire bitboard for the kings. Instead we'll just keep track of two squares, one for each king.
