# Terms and Notation

Conventionally, unless otherwise specified, boards will be displayed from white's perspective.

A *rank* is a row on a chessboard.

```text
8  . . . . . . . . 
7  . . . . . . . . 
6  . . . . . . . . 
5  x x x x x x x x 
4  . . . . . . . . 
3  . . . . . . . . 
2  . . . . . . . . 
1  . . . . . . . . 



the fifth rank
```
<br/>

A *file* is a column on a chessboard.

```text
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 
   . . . . . . x . 

   a b c d e f g h

the g-file
```
<br/>

A *square* is named by its file then rank.

```text
8  . . . . . . . . 
7  . . . . . . . . 
6  . . . . . . . . 
5  . . . . . . x . 
4  . . . . . . . . 
3  . . . . . . . . 
2  . . . . . . . . 
1  . . . . . . . . 
   
   a b c d e f g h

the g5 square
```
<br/>

The coordinates flip if we are viewing the board from black's perspective.

```text
1  . . . . . . . . 
2  . . . . . . . . 
3  . . . . . . . . 
4  . . . . . . . . 
5  . x . . . . . . 
6  . . . . . . . . 
7  . . . . . . . . 
8  . . . . . . . . 
   
   h g f e d c b a

the same g5 square from black's perspective
```
<br/>

Piece moves are represented by the piece letter and the square it moved to.

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . . .    Ng5     5  . . . . . . N .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . N . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . . .            1  . . . . . . . . 
   
   a b c d e f g h               a b c d e f g h

a knight on f3 moving to g5

'N' = knight, 'B' = bishop, 'R' = rook, 'Q' = queen, 'K' = king
```
<br/>

Pawn moves are notated without a letter.

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . . .     g5     5  . . . . . . P .
4  . . . . . . P .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . . .            1  . . . . . . . . 
   
   a b c d e f g h               a b c d e f g h

a white pawn on g4 moving to g5
```
<br/>

Captures are denoted with an `x` between the piece and the square.

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . b .    Nxg5    5  . . . . . . N .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . N . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . . .            1  . . . . . . . . 
   
   a b c d e f g h               a b c d e f g h

a knight captures a bishop on g5
```
<br/>

Pawn captures are denoted by the square from which the pawn originated.

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . q .    fxg5    5  . . . . . . P .
4  . . . . . P . .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . . .            1  . . . . . . . . 
   
   a b c d e f g h               a b c d e f g h

a pawn captures a queen on g5
```
<br/>

If there are multiple moves by the same piece type to the same square, the file and/or rank of origin is used.

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . R . . . . R    Rhg5    5  . . R . . . R .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . . .            1  . . . . . . . . 
   
   a b c d e f g h               a b c d e f g h

8  . . . . . . R .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . . .    R8g5    5  . . . . . . R .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . R .            1  . . . . . . R . 
   
   a b c d e f g h               a b c d e f g h

8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . R . . . . .    Rcg5    5  . . . . . . R .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . . . R .            1  . . . . . . R . 
   
   a b c d e f g h               a b c d e f g h

default to using file
```
<br/>


`O-O` means castle kingside, `O-O-O` means castle queenside

```text
8  . . . . . . . .            8  . . . . . . . . 
7  . . . . . . . .            7  . . . . . . . .    
6  . . . . . . . .            6  . . . . . . . .
5  . . . . . . . .    O-O     5  . . . . . . . .
4  . . . . . . . .   ----->   4  . . . . . . . .
3  . . . . . . . .            3  . . . . . . . . 
2  . . . . . . . .            2  . . . . . . . . 
1  . . . . K . . R            1  . . . . . R K . 
   
   a b c d e f g h               a b c d e f g h
```
<br/>


