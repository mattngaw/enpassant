use super::{Color, Flippable};

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CastlingSide {
    Kingside,
    Queenside,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CastlingRights {
    None,
    Kingside,
    Queenside,
    Both,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Castling(CastlingRights, CastlingRights);

impl Default for Castling {
    fn default() -> Self {
        Castling(CastlingRights::Both, CastlingRights::Both)
    }
}

impl Castling {
    pub fn new() -> Self {
        Castling(CastlingRights::None, CastlingRights::None)
    }

    pub fn get(self, c: Color) -> CastlingRights {
        match c {
            Color::White => self.0,
            Color::Black => self.1,
        }
    }

    pub fn set(&mut self, c: Color, cr: CastlingRights) -> () {
        match c {
            Color::White => self.0 = cr,
            Color::Black => self.1 = cr,
        }
    }

    pub fn clear(&mut self) -> () {
        (self.0, self.1) = (CastlingRights::None, CastlingRights::None)
    }
}

impl Flippable for Castling {
    fn flipped(&self) -> Self {
        Castling(self.1, self.0)
    }
}
