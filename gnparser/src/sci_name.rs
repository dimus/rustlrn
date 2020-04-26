use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SciName {
    pub parsed: bool,
    pub verbatim: String,
    #[serde(rename = "canonicalName")]
    pub canonical_name: Option<Canonical>,
    pub authorship: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Canonical {
    pub full: String,
    pub simple: String,
    pub stem: String,
}
