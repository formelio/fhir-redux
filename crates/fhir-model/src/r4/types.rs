use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::codes;
use crate::r4::resources::Resource;

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
