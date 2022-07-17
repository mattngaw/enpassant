# Board Representation

<center><strong>How do we represent chess in a way that the computer understands?</strong></center></br>

Well, it sounds like we need to encode the rules of chess in our program.

> **FRIEND:** Yep, and we have to make sure we do it perfectly, or else our program will be buggy.

It can't be that hard! There's an 8x8 board of squares, a white and black side, each side gets eight pawns, two rooks, two bishops, two knights, a king and a queen...

> **FRIEND:** What about castling—

That thing where the king and rook swap places? That seems simple enou—

> **FRIEND:** And castling sides.

Right, the king can castle in either direction—

> **FRIEND:** And castling rights.

Um, the king loses castling rights if he moves, and loses castling rights on a side if that side's rook moves.

> **FRIEND:** He also can't castle temporarily if an enemy is attacking the squares that he must move through to get to his castling square.

Wait, what?

> **FRIEND:** And what about promotions?

Uhhh, the pawns can upgrade to other pieces if they reach the end of the board... and there are two ends since white and black go in opposite directions—

> **FRIEND:** Pawns can also move two squares from their starting squares too.

Oh, yeah. Forgot about that.

> **FRIEND:** And pawns that move two can be diagonally captured on the next turn by adjacent enemy pawns.

Huh???

> **FRIEND:** It's called <a href="https://en.wikipedia.org/wiki/En_passant" target="_blank"><em>*en passant*</em></a>.

...

> **FRIEND:** Not as simple as you thought, huh?

There are a *lot* of rules. Maybe I'll have to do a <a href="https://en.wikipedia.org/wiki/Rules_of_chess" target="_blank">refresher</a>.

> **FRIEND:** It's definitely worth the effort, but if you don't know them all, it's no big deal. I'll make sure we cover all of them on our journey.

Epic. Let's move forward then.
