use crate::*;

impl Battle {
    #[allow(dead_code)]
    /// Remove all entry hazards from a side
    pub fn remove_all_hazards(&mut self, side_idx: usize) {
        if side_idx >= self.sides.len() {
            return;
        }

        let side_id = self.sides[side_idx].id_str();
        let hazards = ["stealthrock", "spikes", "toxicspikes", "stickyweb"];

        for hazard_name in hazards {
            let hazard_id = ID::new(hazard_name);
            if self.sides[side_idx].remove_side_condition(&hazard_id) {
                let display_name = match hazard_name {
                    "stealthrock" => "Stealth Rock",
                    "spikes" => "Spikes",
                    "toxicspikes" => "Toxic Spikes",
                    "stickyweb" => "Sticky Web",
                    _ => hazard_name,
                };
                self.add("-sideend", &[side_id.into(), display_name.into()]);
            }
        }
    }
}
