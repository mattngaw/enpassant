//! Keeping track of board state.

use std::mem::transmute;
use std::ops::{Index, IndexMut};

pub mod mailbox;
pub mod castling;

use crate::bits::*;
use mailbox::Mailbox;
use castling::*;

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
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
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
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

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
    /// Gets the square of `c`'s king
    pub fn king_get(&self, c: Color) -> Square {
        self.kings[c as usize]
    }

    /// Sets the square of `c`'s king
    pub fn king_set(&mut self, c: Color, s: Square) -> () {
        self.kings[c as usize] = s
    }
}

impl Index<Square> for Board {
    type Output = Option<Piece>;

    #[inline]
    fn index(&self, index: Square) -> &Self::Output {
        &self.pieces[index]
    }
}

impl IndexMut<Square> for Board {
    #[inline]
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.pieces[index]
    }
}

impl Index<Color> for Board {
    type Output = Bitboard;

    #[inline]
    fn index(&self, index: Color) -> &Self::Output {
        &self.colors[index as usize]
    }
}

impl IndexMut<Color> for Board {
    #[inline]
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.colors[index as usize]
    }
}

impl Index<Role> for Board {
    type Output = Bitboard;

    /// # Requires 
    /// 
    /// `index != Role::King`
    #[inline]
    fn index(&self, index: Role) -> &Self::Output {
        match index {
            Role::King => panic!("Cannot index into roles with Role::King: use Board::king"),
            _ => &self.roles[index as usize],
        }
    }
}

impl IndexMut<Role> for Board {
    #[inline]
    fn index_mut(&mut self, index: Role) -> &mut Self::Output {
        &mut self.roles[index as usize]
    }
}

impl Flippable for Board {
    fn flipped(&self) -> Self {
        Board {
            colors: [self[Color::White].flipped(),
                     self[Color::Black].flipped()],
            roles: [self[Role::Pawn].flipped(),
                    self[Role::Knight].flipped(),
                    self[Role::Bishop].flipped(),
                    self[Role::Rook].flipped(),
                    self[Role::Queen].flipped()],
            kings: [self.king_get(Color::White).flipped(),
                    self.king_get(Color::Black).flipped()],
            pieces: self.pieces.flipped(),
            castling: self.castling.flipped(), 
            turn: self.turn.flipped(),
        }
    }

    fn flip(&mut self) -> () {
        self[Color::White].flip();
        self[Color::Black].flip();
        self[Role::Pawn].flip();
        self[Role::Knight].flip();
        self[Role::Bishop].flip();
        self[Role::Rook].flip();
        self[Role::Queen].flip();
        self.kings[Color::White as usize].flip();
        self.kings[Color::Black as usize].flip(); 
        self.pieces.flip();
        self.castling.flip();
        self.turn.flip();
    }
}