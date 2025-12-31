use crate::*;

/// Dispatch durationCallback for volatile conditions
/// JavaScript calls durationCallback to determine volatile duration
/// Returns Option<i32> - Some(duration) if callback exists, None otherwise
pub fn dispatch_duration_callback(
    battle: &mut Battle,
    condition_id: &str,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> Option<i32> {
    match condition_id {
        // JavaScript source (conditions.js):
        //   partiallytrapped: {
        //       durationCallback(target, source) {
        //           if (source?.hasItem("gripclaw")) return 8;
        //           return this.random(5, 7);
        //       },
        //   }
        "partiallytrapped" => {
            // Check if source has Grip Claw
            if let Some(source_pos) = source_pos {
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    if source_pokemon.item.as_str() == "gripclaw" {
                        return Some(8);
                    }
                }
            }
            // JS: return this.random(5, 7);
            // random(5, 7) returns a value from 5 to 6 inclusive
            let duration = battle.random_with_range(5, 7);
            Some(duration)
        }
        _ => None,
    }
}
