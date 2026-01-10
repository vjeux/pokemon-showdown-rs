//! Result of getMoveTargets

/// Result of getMoveTargets
#[derive(Debug, Clone, Default)]
/// JavaScript equivalent: return type of getMoveTargets (sim/pokemon.ts)
/// 2 fields in JavaScript (inline return type)
pub struct GetMoveTargetsResult {
    /// Target pokemon indices (side_index, position)
    /// JavaScript: targets: Pokemon[]
    /// TODO: Rust uses indices (side_index, position), JavaScript uses Pokemon references
    pub targets: Vec<(usize, usize)>,
    /// Pressure targets for PP deduction
    /// JavaScript: pressureTargets: Pokemon[]
    /// TODO: Rust uses indices (side_index, position), JavaScript uses Pokemon references
    pub pressure_targets: Vec<(usize, usize)>,
}
