use crate::*;

impl Battle {

    /// Join a player to a battle slot
    /// Equivalent to battle.ts join()
    ///
    /// This is a convenience method that wraps setPlayer
    /// Join a player to the battle
    /// Equivalent to battle.ts join() (battle.ts:3261-3264)
    ///
    // TypeScript source:
    // /** @deprecated */
    // 	join(slot: SideID, name: string, avatar: string, team: PokemonSet[] | string | null) {
    // 		this.setPlayer(slot, { name, avatar, team });
    // 		return this.getSide(slot);
    // 	}
    //
    pub fn join(
        &mut self,
        slot: SideID,
        name: &str,
        avatar: &str,
        team: Option<Vec<crate::pokemon::Pokemon>>,
    ) -> Option<usize> {
        // JS: this.setPlayer(slot, { name, avatar, team });
        let options = PlayerOptions {
            name: name.to_string(),
            avatar: if avatar.is_empty() {
                None
            } else {
                Some(avatar.to_string())
            },
            team: crate::battle::TeamFormat::Empty, // Team is handled separately in Rust
            rating: None,
            seed: None,
        };
        self.set_player(slot, options);

        // If a team was provided, replace the side's pokemon
        if let Some(pokemon_team) = team {
            if let Some(side) = self.get_side_mut(slot) {
                side.pokemon = pokemon_team;
            }
        }

        // JS: return this.getSide(slot);
        // In JavaScript, this returns the Side object
        // In Rust, we return the side index since we can't return a reference
        Some(slot.index())
    }
}
