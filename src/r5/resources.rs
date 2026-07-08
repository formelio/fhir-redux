use crate::codes::LinkRelationTypes;
use crate::r5::types;
use crate::resources::{
    CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, PractitionerRole, QuestionnaireResponse,
    ServiceRequest, Task,
};
use crate::types::{Extension, Meta};
use crate::{codes, resource, type_struct};

type_struct!(ActivityDefinition {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::PublicationStatus,
    pub kind: Option<String>,
    pub intent: Option<codes::RequestIntent>,
});

type_struct!(Bundle {
    pub id: String,
    pub r#type: codes::BundleType,
    pub link: Vec<types::BundleLink>,
    pub entry: Vec<types::BundleEntry>,
});

impl Bundle {
    pub fn next(&self) -> Option<String> {
        self.link.iter().find(|link| link.relation == LinkRelationTypes::Next).map(|link| link.url.clone())
    }
}

resource!([
    ActivityDefinition,
    Bundle,
    CarePlan,
    CareTeam,
    Organization,
    Patient,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    QuestionnaireResponse,
    ServiceRequest,
    Task,
]);
