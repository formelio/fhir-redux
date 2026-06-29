use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::codes;
use crate::r5::resources::Resource;
use crate::types::{Annotation, CodeableReference, Extension, Reference};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CarePlanActivity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub modifier_extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub performed_activity: Vec<CodeableReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub progress: Vec<Annotation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub planned_activity_reference: Option<Reference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    pub resource: Resource,
    pub search: BundleEntrySearch,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntrySearch {
    pub mode: codes::SearchEntryMode,
}
