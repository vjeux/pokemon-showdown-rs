use crate::*;
use crate::dex::FormatData;

impl Dex {

    // =========================================================================
    // Format-specific methods
    // =========================================================================

    /// Get all formats
    /// Equivalent to Dex.formats.all() in dex-formats.ts
    pub fn all_formats(&self) -> &[FormatData] {
        &self.formats
    }
}
