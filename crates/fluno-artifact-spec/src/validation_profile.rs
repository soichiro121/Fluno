use crate::manifest::ArtifactOutputMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactValidationProfile {
    pub validation_kind: String,
    pub tolerance_abs: f64,
    pub tolerance_rel: f64,
    pub reference_digest: Option<String>,
    pub output_mode: ArtifactOutputMode,
    pub partial_materialized_output: bool,
    pub publishable_requires_validation: bool,
}
