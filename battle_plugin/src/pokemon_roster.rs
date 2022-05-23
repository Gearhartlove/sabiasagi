use core::fmt;
use fmt::Display;
use std::fmt::Formatter;

impl Display for Pokemon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// could do: parse the gen 1 pokemon list to get stats,
// create hash "enum" -> "fighter struct"
#[derive(Debug)]
pub enum Pokemon {
    Weedle,
    Charmeleon,
}