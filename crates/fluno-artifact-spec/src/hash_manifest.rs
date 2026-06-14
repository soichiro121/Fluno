use crate::errors::ArtifactSpecResult;
use crate::manifest::ArtifactManifest;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn sha256_text(text: &str) -> String {
    sha256_bytes(text.as_bytes())
}

pub fn canonical_json_hash<T: Serialize>(value: &T) -> ArtifactSpecResult<String> {
    let bytes = serde_json::to_vec(value)?;
    Ok(sha256_bytes(&bytes))
}

pub fn manifest_integrity_hash(manifest: &ArtifactManifest) -> ArtifactSpecResult<String> {
    let mut canonical = manifest.clone();
    canonical.artifact_manifest_hash = None;
    canonical_json_hash(&canonical)
}

pub fn package_file_manifest_hash(lines: &str) -> String {
    sha256_text(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_are_stable() {
        assert_eq!(
            sha256_text("fluno"),
            "d1f851099daabaaad724f00552ae688e58aa7df085fd64578cd673041ce18afc"
        );
    }
}
