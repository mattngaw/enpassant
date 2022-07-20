use super::{Square, Piece};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    pub fn new() -> Mailbox {
        Mailbox([None; 64])
    }

    pub fn get(self, s: Square) -> Option<Piece> {
        self.0[s.index()]
    }

    pub fn set(&mut self, s: Square, p: Option<Piece>) -> () {
        self.0[s.index()] = p;
    }

    pub fn flipped(self) -> Self {
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