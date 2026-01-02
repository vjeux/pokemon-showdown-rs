// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl<'a> BattleActions<'a> {
    pub fn new(dex: &'a Dex, gen: u8) -> Self {
        Self { dex, gen }
    }
}
