use serde::{Deserialize, Serialize};

/// Represents an anonymized summary of a citizen's financial activity.
///
/// Implements: credential-hierarchy.md#Section-2-Financial-Credentials
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FinancialSummary {
    /// Total transaction volume for the year (anonymized).
    pub annual_volume: u128,
    /// Whether the tax for this volume has been cleared.
    pub tax_cleared: bool,
    /// The currency used (e.g., "NGN").
    pub currency: String,
}
