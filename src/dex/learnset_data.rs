//! Learnset data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Event data for learnsets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: EventInfo (sim/dex-species.ts)
/// 4 fields in JavaScript
pub struct EventData {
    /// Generation of the event
    /// JavaScript: generation?: number
    #[serde(default)]
    pub generation: Option<u8>,
    /// Level of the event Pokemon
    /// JavaScript: level?: number
    #[serde(default)]
    pub level: Option<u8>,
    /// Moves the event Pokemon knows
    /// JavaScript: moves?: string[]
    #[serde(default)]
    pub moves: Vec<String>,
    /// Source/description of the event
    /// JavaScript: source?: string
    #[serde(default)]
    pub source: Option<String>,
}

/// Learnset data for a single species
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// JavaScript equivalent: LearnsetData (sim/dex-species.ts)
/// Fields: learnset, eventData, eventOnly, encounters, exists
pub struct LearnsetData {
    /// Map from move ID to learn methods (e.g., "9M", "8L15", "7E")
    /// JavaScript: learnset: { [moveid: string]: string[] }
    #[serde(default)]
    pub learnset: HashMap<String, Vec<String>>,
    /// Event-only moves
    /// JavaScript: eventData?: EventInfo[]
    #[serde(default)]
    pub event_data: Option<Vec<EventData>>,
    /// Is this Pokemon event-only?
    /// JavaScript: eventOnly?: boolean
    #[serde(default)]
    pub event_only: Option<bool>,
}
