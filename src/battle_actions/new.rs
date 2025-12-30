use crate::*;

impl<'a> BattleActions<'a> {
    pub fn new(dex: &'a Dex, gen: u8) -> Self {
        Self { dex, gen }
    }
}
