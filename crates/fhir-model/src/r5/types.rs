use crate::r5::resources::Resource;
use crate::types::{Annotation, CodeableReference, Extension, Reference};
use crate::{codes, type_struct};

type_struct!(CarePlanActivity {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub performed_activity: Vec<CodeableReference>,
    pub progress: Vec<Annotation>,
    pub planned_activity_reference: Option<Reference>,
});

type_struct!(BundleEntry {
    pub resource: Resource,
    pub search: BundleEntrySearch,
});

type_struct!(BundleEntrySearch {
    pub mode: codes::SearchEntryMode,
});
