use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputSpec {
    pub name: String,
    pub dtype: String,
    pub rank: usize,
    pub shape: Vec<usize>,
    #[serde(default)]
    pub runtime_parameter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputSchema {
    pub schema_version: String,
    pub inputs: Vec<InputSpec>,
}

impl InputSchema {
    pub fn scalar_i64(name: impl Into<String>) -> Self {
        Self {
            schema_version: "fluno_input_schema.v1".to_string(),
            inputs: vec![InputSpec {
                name: name.into(),
                dtype: "i64".to_string(),
                rank: 0,
                shape: Vec::new(),
                runtime_parameter: true,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputSchema {
    pub schema_version: String,
    pub output_mode: String,
    pub partial_materialized_output: bool,
    pub output_vector_len: usize,
    pub output_dtype: String,
    pub digest_dtype: String,
}
