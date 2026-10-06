use crate::r4::resources::Resource;
use crate::types::{CodeableConcept, Extension, Period, Reference};
use crate::{codes, type_struct};

type_struct!(BundleLink {
    pub relation: codes::LinkRelationTypes,
    pub url: String,
});

type_struct!(BundleEntry {
    pub full_url: Option<String>,
    pub resource: Resource,
    pub search: BundleEntrySearch,
});

type_struct!(BundleEntrySearch {
    pub mode: codes::SearchEntryMode,
});

type_struct!(ConsentPolicy {
    pub uri: Option<String>,
});

type_struct!(ConsentProvision {
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub r#type: Option<codes::ConsentProvisionType>,
    pub period: Option<Period>,
    pub actor: Vec<ConsentProvisionActor>,
    pub code: Vec<CodeableConcept>,
});

type_struct!(ConsentProvisionActor {
    pub extension: Vec<Extension>,
    pub role: CodeableConcept,
    pub reference: Reference,
});

type_struct!(EncounterParticipant {
    pub extension: Vec<Extension>,
    pub r#type: Vec<CodeableConcept>,
    pub individual: Option<Reference>,
});

type_struct!(ProcedurePerformer {
    pub extension: Vec<Extension>,
    pub actor: Reference,
});

type_struct!(DeviceUdiCarrier {
    pub device_identifier: Option<String>,
    pub issuer: Option<String>,
    pub carrier_hrf: Option<String>,
});

type_struct!(DeviceDeviceName {
    pub name: String,
    pub r#type: String,
});
