use crate::*;

impl Battle {

    // =========================================================================
    // Targeting Methods (ported from battle.ts)
    // =========================================================================

    /// Get the location of a target Pokemon from a source Pokemon's perspective
    /// Equivalent to pokemon.ts getLocOf() (pokemon.ts:768-773)
    ///
    /// JS: getLocOf(target: Pokemon) {
    ///   const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
    ///   const position = target.position + positionOffset + 1;
    ///   const sameHalf = (this.side.n % 2) === (target.side.n % 2);
    ///   return sameHalf ? -position : position;
    /// }
    pub fn get_loc_of(&self, source: (usize, usize), target: (usize, usize)) -> i32 {
        let (target_side_idx, target_poke_idx) = target;

        if let Some(target_side) = self.sides.get(target_side_idx) {
            if let Some(target_pokemon) = target_side.pokemon.get(target_poke_idx) {
                // const positionOffset = Math.floor(target.side.n / 2) * target.side.active.length;
                let position_offset = (target_side.n / 2) * target_side.active.len();
                // const position = target.position + positionOffset + 1;
                let position = (target_pokemon.position + position_offset + 1) as i32;

                // const sameHalf = (this.side.n % 2) === (target.side.n % 2);
                if let Some(source_side) = self.sides.get(source.0) {
                    let same_half = (source_side.n % 2) == (target_side.n % 2);
                    // return sameHalf ? -position : position;
                    return if same_half { -position } else { position };
                }
            }
        }
        0
    }
}
