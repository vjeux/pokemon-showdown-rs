use crate::*;

impl Battle {

    /// Get Pokemon at a specific slot location
    /// Get Pokemon at a slot string (e.g., "p1a", "p2b")
    /// Equivalent to battle.ts getAtSlot()
    //
    // 	getAtSlot(slot: PokemonSlot): Pokemon;
    //
    pub fn get_at_slot(&self, slot: Option<&str>) -> Option<&Pokemon> {
        // JS: if (!slot) return null;
        let slot_str = slot?;

        if slot_str.len() < 3 {
            return None;
        }

        // JS: const side = this.sides[slot.charCodeAt(1) - 49]; // 49 is '1'
        let side_char = slot_str.chars().nth(1)?;
        let side_idx = (side_char as i32).checked_sub(49)? as usize; // 49 is '1'

        // JS: const position = slot.charCodeAt(2) - 97; // 97 is 'a'
        let pos_char = slot_str.chars().nth(2)?;
        let position = (pos_char as i32).checked_sub(97)? as usize; // 97 is 'a'

        // JS: const positionOffset = Math.floor(side.n / 2) * side.active.length;
        // JS: return side.active[position - positionOffset];
        let side = self.sides.get(side_idx)?;
        let position_offset = (side.n / 2) * side.active.len();
        let adjusted_position = position.checked_sub(position_offset)?;

        let poke_idx = side.active.get(adjusted_position)?.as_ref()?;
        side.pokemon.get(*poke_idx)
    }
}
