use crate::*;

impl Pokemon {

    /// Copy volatiles from another Pokemon (for Baton Pass)
    pub fn copy_volatile_from_full(&mut self, source: &Pokemon, is_shed_tail: bool) {
        // Copy boosts unless Shed Tail
        if !is_shed_tail {
            self.boosts = source.boosts;
        }

        // List of volatiles that can be copied
        let copyable_volatiles = [
            "aquaring",
            "confusion",
            "curse",
            "embargo",
            "focusenergy",
            "gmaxchistrike",
            "healblock",
            "ingrain",
            "laserfocus",
            "leechseed",
            "magnetrise",
            "perishsong",
            "powertrick",
            "substitute",
            "telekinesis",
            "torment",
        ];

        for volatile_name in copyable_volatiles {
            if is_shed_tail && volatile_name != "substitute" {
                continue;
            }
            let id = ID::new(volatile_name);
            if let Some(state) = source.get_volatile(&id) {
                self.volatiles.insert(id, state.clone());
            }
        }
    }
}
