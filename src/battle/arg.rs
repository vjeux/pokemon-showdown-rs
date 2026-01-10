//! Arg Type

use crate::pokemon::Pokemon;
use super::SplitMessage;

/// Argument type for battle.add() - can be a Pokemon reference or a string
/// This allows mixing types like: battle.add("-activate", &[pokemon.into(), "ability: Immunity".into()])
/// TODO: Not in JavaScript - Rust-specific enum for type-safe protocol message arguments
/// JavaScript uses spread arguments with mixed types (Pokemon | string | Function)
pub enum Arg<'a> {
    Pokemon(&'a Pokemon),
    Str(&'a str),
    String(String),
    /// Function that returns split message for side-specific content
    /// JavaScript equivalent: () => { side: SideID, secret: string, shared: string }
    SplitFn(Box<dyn Fn() -> SplitMessage + 'a>),
}

impl<'a> From<&'a Pokemon> for Arg<'a> {
    fn from(p: &'a Pokemon) -> Self {
        Arg::Pokemon(p)
    }
}

impl<'a> From<&'a str> for Arg<'a> {
    fn from(s: &'a str) -> Self {
        Arg::Str(s)
    }
}

impl<'a> From<String> for Arg<'a> {
    fn from(s: String) -> Self {
        Arg::String(s)
    }
}

impl std::fmt::Display for Arg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Pokemon(p) => write!(f, "{}", p),
            Arg::Str(s) => write!(f, "{}", s),
            Arg::String(s) => write!(f, "{}", s),
            Arg::SplitFn(_) => write!(f, "[split function]"),
        }
    }
}
