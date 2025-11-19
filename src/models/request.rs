use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MergeMode {
    Before,  // Overload runs before base
    After,   // Overload runs after base
}

impl Default for MergeMode {
    fn default() -> Self {
        MergeMode::Before
    }
}
