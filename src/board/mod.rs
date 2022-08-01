//! Keeping track of board state.

use std::mem::transmute;

pub mod mailbox;
pub mod castling;

use crate::bits::*;
pub use mailbox::Mailbox;
pub use castling::*;

/// Component of a chess position that must be flipped when switching sides.
pub trait Flippable
where Self: Copy + Sized
{
    /// Returns the flipped version of an object.
    fn flipped(&self) -> Self;

    /// Flips an object in place.
    fn flip(&mut self) -> () {
        *self = self.flipped()
    }
}

/// A column on a chessboard.
#[repr(u8)]
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
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

impl File {
    pub fn new(x: u8) -> Self {
        assert!(x < 8);
        Self::new_unchecked(x)
    }

    pub fn new_unchecked(x: u8) -> Self {
        debug_assert!(x < 8);
        unsafe { transmute::<u8, File>(x) }
    }
}

/// A row on a chessboard.
#[repr(u8)]
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Rank { 
    First = 0,
    Second = 1, 
    Third = 2,
    Fourth = 3, 
    Fifth = 4, 
    Sixth = 5, 
    Seventh = 6, 
    Eighth = 7, 
}

impl Rank {
    pub fn new(x: u8) -> Self {
        assert!(x < 8);
        Self::new_unchecked(x)
    }

    pub fn new_unchecked(x: u8) -> Self {
        debug_assert!(x < 8);
        unsafe { transmute::<u8, Rank>(x) }
    }
}

/// White or black.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Flippable for Color {
    #[inline]
    fn flipped(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// The type of piece. 
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Direction relative to perspective.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

/// Combination of [`Color`] and [`Role`].
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece(Color, Role);

/// Full representation of a chessboard (not the game state, see
/// [`Position`](#)).
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Board {
    turn: Color,
    colors: [Bitboard; 2],
    roles: [Bitboard; 5],
    kings: [Square; 2],
    pieces: Mailbox,
    pub castling: Castling,
}

impl Board {
    pub fn place(&mut self, s: Square, p: Piece) -> bool {
        match self.pieces[s] {
            Some(Piece(c, r)) => {
                match r {
                    Role::King => self.king_set(c, s),
                    _ => self.piece_set(p, s),
                }
                true
            }
            None => false,
        }
    }

    pub fn remove(&mut self, s: Square) -> Option<Piece> {
        self.pieces[s].and_then(|p| {
            match p {
                Piece(_, Role::King) => None,
                Piece(c, r) => {
                    let result = self.pieces[s];
                    self.pieces[s] = None;
                    self.colors[c as usize].remove(s);
                    self.roles[r as usize].remove(s);
                    result 
                }
            }
        })
    }

    pub fn replace(&mut self, s: Square, p: Piece) -> Option<Piece> {
        let result = self.remove(s);
        match p {
            Piece(c, Role::King) => self.king_set(c, s),
            _ => self.piece_set(p, s),
        }
        result
    }

    pub fn r#move(&mut self, from: Square, to: Square) -> Option<Piece> {
        let moved = self.remove(from);
        self.replace(to, moved.expect("moved an empty square"))
    }

    /// Gets the square of `c`'s king
    fn king_get(&self, c: Color) -> Square {
        self.kings[c as usize]
    }
    
    /// Sets the square of `c`'s king
    fn king_set(&mut self, c: Color, s: Square) -> () {
        let k = self.king_get(c);
        self.pieces[k] = None;
        self.pieces[s] = Some(Piece(c, Role::King));
        self.kings[c as usize] = s;
        self.colors[c as usize].add(s);
    }

    fn piece_set(&mut self, p: Piece, s: Square) -> () {
        self.colors[p.0 as usize].add(s); 
        self.roles[p.1 as usize].add(s);
        self.pieces[s] = Some(p);
    }
}

impl Flippable for Board {
    fn flipped(&self) -> Self {
        Board {
            colors: [self.colors[Color::White as usize].flipped(),
                     self.colors[Color::Black as usize].flipped()],
            roles: [self.roles[Role::Pawn as usize].flipped(),
                    self.roles[Role::Knight as usize].flipped(),
                    self.roles[Role::Bishop as usize].flipped(),
                    self.roles[Role::Rook as usize].flipped(),
                    self.roles[Role::Queen as usize].flipped()],
            kings: [self.king_get(Color::White).flipped(),
                    self.king_get(Color::Black).flipped()],
            pieces: self.pieces.flipped(),
            castling: self.castling.flipped(), 
            turn: self.turn.flipped(),
        }
    }

    fn flip(&mut self) -> () {
        self.colors[Color::White as usize].flip();
        self.colors[Color::Black as usize].flip();
        self.roles[Role::Pawn as usize].flip();
        self.roles[Role::Knight as usize].flip();
        self.roles[Role::Bishop as usize].flip();
        self.roles[Role::Rook as usize].flip();
        self.roles[Role::Queen as usize].flip();
        self.kings[Color::White as usize].flip();
        self.kings[Color::Black as usize].flip(); 
        self.pieces.flip();
        self.castling.flip();
        self.turn.flip();
    }
}