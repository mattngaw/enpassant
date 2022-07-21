use std::mem::transmute;

pub mod bits;
pub mod mailbox;
pub mod castling;

use bits::*;
use mailbox::Mailbox;
use castling::*;

/// A column on a chessboard
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

/// A row on a chessboard
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

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

pub struct Board {
    colors: Bitboard,
    roles: Bitboard,
    pieces: Mailbox,
}

pub trait Flippable
where Self: Copy + Sized
{
    fn flipped(&self) -> Self;
    fn flip(&mut self) -> () {
        *self = self.flipped()
    }
}