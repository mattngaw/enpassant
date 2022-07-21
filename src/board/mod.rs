use std::mem::transmute;

pub mod mailbox;
pub mod castling;

use crate::bits::*;
use mailbox::Mailbox;
use castling::*;

pub trait Flippable
where Self: Copy + Sized
{
    fn flipped(&self) -> Self;
    fn flip(&mut self) -> () {
        *self = self.flipped()
    }
}

pub trait Index {
    fn index(&self) -> usize;
}

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

impl Index for File {
    #[inline]
    fn index(&self) -> usize {
        *self as usize
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

impl Index for Rank {
    #[inline]
    fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Index for Color {
    #[inline]
    fn index(&self) -> usize {
        *self as usize
    }
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

impl Index for Role {
    #[inline]
    fn index(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Index for Direction {
    #[inline]
    fn index(&self) -> usize {
        *self as usize
    }
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

