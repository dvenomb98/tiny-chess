// src/tests/mod.rs
// This makes tests/ a module within your crate

#[cfg(test)]
mod bishop;

#[cfg(test)]
mod fen;

#[cfg(test)]
mod king;

#[cfg(test)]
mod knight;

#[cfg(test)]
mod pawn;

#[cfg(test)]
mod queen;

#[cfg(test)]
mod rook;

#[cfg(test)]
mod square;

#[cfg(test)]
mod state;