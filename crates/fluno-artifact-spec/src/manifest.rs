use crate::errors::{ArtifactSpecError, ArtifactSpecResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const FLUNO_ARTIFACT_FORMAT_VERSION: &str = "fluno_artifact.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactOutputMode {
    Digest,
    PartialSummaryVector,
    FullStateVector,
}

impl ArtifactOutputMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactOutputMode::Digest => "digest",
            ArtifactOutputMode::PartialSummaryVector => "partial_summary_vector",
            ArtifactOutputMode::FullStateVector => "full_state_vector",
        }
    }

    pub fn parse(text: &str) -> ArtifactSpecResult<Self> {
        match text {
            "digest" => Ok(Self::Digest),
            "partial_summary_vector" | "partial-summary-vector" => Ok(Self::PartialSummaryVector),
            "full_state_vector" | "full-state-vector" => Ok(Self::FullStateVector),
            other => Err(ArtifactSpecError::InvalidOutputMode(other.to_string())),
        }
    }

    pub fn from_str(text: &str) -> Result<Self, String> {
        Self::parse(text).map_err(|err| err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactLicensePolicy {
    pub mode: String,
    pub reverse_engineering_prohibited: bool,
    pub third_party_redistribution_prohibited: bool,
}

impl Default for ArtifactLicensePolicy {
    fn default() -> Self {
        Self {
            mode: "internal".to_string(),
            reverse_engineering_prohibited: true,
            third_party_redistribution_prohibited: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactManifest {
    pub artifact_format_version: String,
    pub artifact_id: String,
    pub build_id: String,
    pub customer_id_hash: Option<String>,
    pub workload_name: String,
    pub region_name: String,
    pub region_hash: String,
    pub core_ir_hash: String,
    pub generated_mlir_hash: String,
    pub abi_signature_hash: String,
    pub compiled_library_hash: String,
    #[serde(default)]
    pub digest_library_path: Option<String>,
    #[serde(default)]
    pub digest_library_hash: Option<String>,
    #[serde(default)]
    pub digest_entry_symbol: Option<String>,
    #[serde(default)]
    pub digest_abi_signature_hash: Option<String>,
    #[serde(default)]
    pub vector_library_hash: Option<String>,
    #[serde(default)]
    pub vector_abi_signature_hash: Option<String>,
    #[serde(default)]
    pub materialized_mlir_hash: Option<String>,
    #[serde(default)]
    pub materialized_output_schema_hash: Option<String>,
    #[serde(default)]
    pub input_schema_hash: Option<String>,
    #[serde(default)]
    pub output_schema_hash: Option<String>,
    #[serde(default)]
    pub validation_profile_hash: Option<String>,
    #[serde(default)]
    pub allowed_shape_schema_hash: Option<String>,
    #[serde(default)]
    pub artifact_manifest_hash: Option<String>,
    #[serde(default)]
    pub package_file_manifest_hash: Option<String>,
    pub target: String,
    pub os: String,
    pub arch: String,
    pub backend: String,
    pub compiler_flags_hash: String,
    pub fluno_version: String,
    pub git_commit: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub allowed_input_shapes: Vec<Value>,
    pub output_mode: ArtifactOutputMode,
    pub partial_materialized_output: bool,
    pub output_vector_len: usize,
    #[serde(default)]
    pub full_state_vector_validated: bool,
    #[serde(default)]
    pub materialized_output_validated: bool,
    #[serde(default)]
    pub materialized_output_scope: Option<String>,
    pub license_policy: ArtifactLicensePolicy,
    pub library_path: Option<String>,
    pub entry_symbol: String,
    pub vector_library_path: Option<String>,
    pub vector_entry_symbol: Option<String>,
    pub cache_key: String,
    #[serde(default)]
    pub runtime_parameterized_shapes: bool,
    #[serde(default)]
    pub shape_reuse_policy: Option<String>,
    #[serde(default)]
    pub symbols_stripped: bool,
    #[serde(default)]
    pub strip_status: Option<String>,
    #[serde(default)]
    pub strip_skip_reason: Option<String>,
    #[serde(default)]
    pub debug_files_included: bool,
    #[serde(default)]
    pub internal_dumps_included: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageFileEntry {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackageFileManifest {
    pub files: Vec<PackageFileEntry>,
}
