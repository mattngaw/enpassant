//! Struct for keeping track of what's on each square.

use std::ops::{Index, IndexMut};

use super::{Piece, Flippable}; 
use crate::bits::Square;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox([None; 64])
    }

    pub fn set(&mut self, s: Square, p: Option<Piece>) -> () {
        self.0[usize::from(s)] = p;
    }
}

impl Flippable for Mailbox {
    fn flipped(&self) -> Self {
        let mut flipped = Mailbox::new();
        for i in 0..64usize {
            flipped.0[i] = self.0[63 - i];
        }
        flipped
    }
}

impl IntoIterator for Mailbox {
    type Item = Option<Piece>;

    type IntoIter = std::array::IntoIter<Self::Item, 64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Index<Square> for Mailbox {
    type Output = Option<Piece>;

    fn index(&self, s: Square) -> &Self::Output {
        &self.0[usize::from(s)]
    }
}

impl IndexMut<Square> for Mailbox {
    fn index_mut(&mut self, s: Square) -> &mut Self::Output {
        &mut self.0[usize::from(s)]
    }
}
