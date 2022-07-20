# Board Representation

<center><strong>How do we represent chess in a way that the computer understands?</strong></center></br>

How would you answer this question?


> **FRIEND:** Well, it sounds like we need to encode the rules of chess in our program.

Yep, and it's important that we do it perfectly, or else our program will be buggy. It's also worth considering efficiency, since a faster chess engine is a strong one.

> **FRIEND:** Well, it shouldn't be too hard. There's an 8x8 board of squares, a white and black side, each side gets eight pawns, two rooks, two bishops, two knights, a king and a queen...

What about castling—

> **FRIEND:** That thing where the king and rook swap places? That seems simple enou—

There are two rooks.

> **FRIEND:** Right, the king can castle in either direction—

Not always though.

> **FRIEND:** Well, the king loses castling rights if he moves, and he loses castling rights on a side if the rook on that side moves.

He also can't castle temporarily if an enemy is attacking the squares that he must move through to get to his castling square.

> **FRIEND:** Wait, wha—

And what about promotions?

> **FRIEND:** Uhhh, the pawns can upgrade to other pieces if they reach the end of the board... and there are two ends since white and black go in opposite directions—

Pawns can also move two squares from their starting squares too.

> **FRIEND:** Oh, yeah. Forgot about that.

And pawns that move two can be diagonally captured on and only on the next turn by adjacent enemy pawns.

> **FRIEND:** Huh???

It's called <a href="https://en.wikipedia.org/wiki/En_passant" target="_blank"><em>*en passant*</em></a>.

> **FRIEND:** ...

Not as simple as you thought, huh?

> **FRIEND:** There are a *lot* of little rules. Maybe I'll have to do a <a href="https://en.wikipedia.org/wiki/Rules_of_chess" target="_blank">refresher</a>.

It's definitely worth the effort. Still, if you don't know them all, it's no big deal. I'll make sure we cover all of them on our journey. However, it will be useful to know [some chess terminalogy and notation](../termsandnotation.md). Let's move forward then.
