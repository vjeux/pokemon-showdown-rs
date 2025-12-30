use crate::*;
use crate::battle_actions::ActiveMove;
use crate::dex::Accuracy;

impl Dex {

    /// Get an ActiveMove from a move name or ID
    /// Equivalent to Dex.getActiveMove() in dex.ts
    ///
    /// TypeScript source:
    /// ```typescript
    /// getActiveMove(move: Move | string): ActiveMove {
    ///     if (move && typeof (move as ActiveMove).hit === 'number') return move as ActiveMove;
    ///     move = this.moves.get(move);
    ///     const moveCopy: ActiveMove = this.deepClone(move);
    ///     moveCopy.hit = 0;
    ///     return moveCopy;
    /// }
    /// ```
    ///
    /// Creates a mutable copy of move data suitable for use in battle
    pub fn get_active_move(&self, name: &str) -> Option<ActiveMove> {
        let move_data = self.moves().get(name)?;

        // Convert MoveData to ActiveMove
        let active_move = ActiveMove {
            id: move_data.id.clone(),
            name: move_data.name.clone(),
            base_power: move_data.base_power,
            category: move_data.category.clone(),
            move_type: move_data.move_type.clone(),
            accuracy: match move_data.accuracy {
                Accuracy::Percent(acc) => acc,
                Accuracy::AlwaysHits => 0, // 0 means always hits
            },
            priority: move_data.priority,
            target: move_data.target.clone(),
            flags: Self::convert_move_flags(&move_data.flags),

            // Active state - initialize to defaults
            hit: 0,
            total_damage: 0,
            spread_hit: false,
            is_external: false,
            is_z: false,
            is_max: false,
            is_z_or_max_powered: false,
            prankster_boosted: false,
            has_bounced: false,
            source_effect: None,
            ignore_ability: false,
            ignore_immunity: None,
            ignore_accuracy: false,
            ignore_evasion: false,
            ignore_defensive: false,
            ignore_offensive: false,
            ignore_negative_offensive: false,
            ignore_positive_defensive: false,
            override_offensive_stat: None,
            will_crit: None,
            force_stab: false,
            crit_ratio: 0,
            crit_modifier: None,
            self_switch: None,
            self_boost: None,
            has_sheer_force: false,
            mindblown_recoil: false,
            struggle_recoil: false,
            self_dropped: false,
            smart_target: move_data.smart_target,
            stellar_boosted: false,
            multi_hit: None,
            multi_hit_type: None,
            multi_accuracy: false,
            ohko: None,
            always_hit: matches!(move_data.accuracy, Accuracy::AlwaysHits),
            breaks_protect: false,
            steals_boosts: false,
            force_switch: false,
            self_destruct: move_data.selfdestruct.clone(),
            tracks_target: move_data.tracks_target.unwrap_or(false),
            base_move: None,
            max_move: None,
            z_move: None,
            sleep_usable: false,

            // Special move fields
            non_ghost_target: None,
            will_change_forme: false,

            // Secondary effects - convert from move_data.secondary
            secondaries: if let Some(ref sec) = move_data.secondary {
                vec![Self::convert_secondary(sec)]
            } else {
                Vec::new()
            },
            self_effect: None,

            // Move data effects - copy from move_data
            boosts: move_data
                .boosts
                .as_ref()
                .map(Self::convert_boosts_hash_to_table),
            heal: move_data.heal,
            status: move_data.status.clone(),
            force_status: None,
            volatile_status: move_data.volatile_status.clone(),
            side_condition: None,
            slot_condition: None,
            weather: None,
            terrain: None,
            pseudo_weather: None,

            // Recoil
            recoil: move_data.recoil,

            // Infiltrates flag (default to false)
            infiltrates: false,

            // Hit targets (populated during execution)
            hit_targets: Vec::new(),
        };

        Some(active_move)
    }
}
