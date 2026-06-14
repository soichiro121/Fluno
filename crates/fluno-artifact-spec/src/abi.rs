use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactAbiType {
    Unit,
    Bool,
    I64,
    F64,
    Tuple(Vec<ArtifactAbiType>),
    StaticTensor {
        element: ArtifactScalarAbiType,
        shape: Vec<usize>,
        layout: ArtifactTensorLayout,
    },
    DynamicTensorDescriptor,
    DistributionHandle,
    EventHandle,
    SignalHandle,
    StreamHandle,
    FunctionHandle,
    RuntimeHandle(String),
    Opaque(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactScalarAbiType {
    Bool,
    I64,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactTensorLayout {
    RowMajor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactAbiSignature {
    pub params: Vec<ArtifactAbiType>,
    pub results: Vec<ArtifactAbiType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactAbiSignatureFile {
    pub abi_signature_hash: String,
    pub digest_signature: Value,
    pub vector_signature: Option<Value>,
    pub call_plan: Value,
}
