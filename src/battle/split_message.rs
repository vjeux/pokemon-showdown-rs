//! Split Message

use crate::dex_data::SideID;

/// JavaScript equivalent: { side: SideID, secret: string, shared: string }
/// 3 fields in JavaScript
pub struct SplitMessage {
    /// Side ID for side-specific content
    /// JavaScript: side: SideID
    pub side: SideID,
    /// Secret message (shown only to this side)
    /// JavaScript: secret: string
    pub secret: String,
    /// Shared message (shown to all players)
    /// JavaScript: shared: string
    pub shared: String,
}
