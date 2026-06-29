use crate::r4::resources::Resource;
use crate::{codes, type_struct};

type_struct!(BundleEntry {
    pub resource: Resource,
    pub search: BundleEntrySearch,
});

type_struct!(BundleEntrySearch {
    pub mode: codes::SearchEntryMode,
});
