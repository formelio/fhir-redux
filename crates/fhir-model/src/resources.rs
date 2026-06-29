use crate::date_time::DateTime;
use crate::r5::resources::Resource;
use crate::r5::types::CarePlanActivity;
use crate::types::{
    Annotation, CareTeamParticipant, CodeableConcept, ContactPoint, ExtendedContactDetail, Extension, HumanName,
    Identifier, Meta, Period, PlanDefinitionAction, Reference, TaskInput, TaskOutput,
};
use crate::{codes, resource_struct};

resource_struct!(CarePlan {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub instantiates_canonical: Vec<String>,
    pub status: codes::RequestStatus,
    pub intent: String,
    pub category: Vec<CodeableConcept>,
    pub title: Option<String>,
    pub subject: Reference,
    pub period: Option<Period>,
    pub created: Option<DateTime>,
    pub custodian: Option<Reference>,
    pub care_team: Vec<Reference>,
    pub supporting_info: Vec<Reference>,
    pub activity: Vec<CarePlanActivity>,
    pub note: Vec<Annotation>,
});

resource_struct!(CareTeam {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub participant: Vec<CareTeamParticipant>,
});

resource_struct!(Organization {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub identifier: Vec<Identifier>,
    pub active: Option<bool>,
    pub r#type: Vec<CodeableConcept>,
    pub name: Option<String>,
    pub contact: Vec<ExtendedContactDetail>,
});

resource_struct!(Patient {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Vec<Identifier>,
    pub active: Option<bool>,
    pub name: Vec<HumanName>,
});

resource_struct!(PlanDefinition {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub contained: Vec<Resource>,
    pub url: Option<String>,
    pub version: Option<String>,
    pub title: Option<String>,
    pub status: codes::PublicationStatus,
    pub action: Vec<PlanDefinitionAction>,
});

resource_struct!(Practitioner {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Vec<Identifier>,
    pub active: Option<bool>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
});

resource_struct!(ActivityDefinition {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub title: Option<String>,
    pub status: codes::PublicationStatus,
    pub kind: Option<String>,
    pub intent: Option<codes::RequestIntent>,
});

resource_struct!(ServiceRequest {
    pub id: String,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub based_on: Vec<Reference>,
    pub status: codes::RequestStatus,
    pub intent: codes::RequestIntent,
    pub subject: Reference,
    // occurrence: Option<ServiceRequestOccurrence>,
    pub authored_on: Option<DateTime>,
    pub requester: Option<Reference>,
});

resource_struct!(Task {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub instantiates_canonical: Option<String>,
    pub based_on: Vec<Reference>,
    pub status: codes::TaskStatus,
    pub business_status: Option<CodeableConcept>,
    pub intent: String,
    pub r#for: Option<Reference>,
    pub requested_period: Option<Period>,
    pub authored_on: Option<DateTime>,
    pub last_modified: Option<DateTime>,
    pub requester: Option<Reference>,
    pub owner: Option<Reference>,
    pub input: Vec<TaskInput>,
    pub output: Vec<TaskOutput>,
});

resource_struct!(PractitionerRole {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub active: Option<bool>,
    pub practitioner: Option<Reference>,
    pub organization: Option<Reference>,
    pub code: Vec<CodeableConcept>,
    pub specialty: Vec<CodeableConcept>,
});

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    ActivityDefinition,
    Bundle,
    CarePlan,
    CareTeam,
    Endpoint,
    Organization,
    Patient,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    ServiceRequest,
    Task,
}
