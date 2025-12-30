use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Get number of hits for multi-hit moves
    pub fn get_multi_hit_count(&mut self, move_id: &ID) -> i32 {
        // Extract multihit data before calling mutable method
        let multihit_data = if let Some(move_def) = self.dex.get_move(move_id.as_str()) {
            move_def.multihit.clone()
        } else {
            None
        };

        // Process multihit data
        if let Some(multihit) = multihit_data {
            match multihit {
                crate::dex::Multihit::Fixed(n) => return n,
                crate::dex::Multihit::Range(min, max) => {
                    // Variable hit move - distribute as: 35% min, 35% min+1, 15% max-1, 15% max
                    let roll = self.random(100);
                    return if roll < 35 {
                        min
                    } else if roll < 70 {
                        (min + 1).min(max)
                    } else if roll < 85 {
                        (max - 1).max(min)
                    } else {
                        max
                    };
                }
            }
        }

        1 // Default: single hit
    }
}
