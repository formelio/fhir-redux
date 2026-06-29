use crate::date_time::DateTime;
use crate::r5::resources::Resource;
use crate::r5::types::CarePlanActivity;
use crate::types::{
    Annotation, CareTeamParticipant, CodeableConcept, ContactPoint, ExtendedContactDetail, Extension, HumanName,
    Identifier, Meta, Period, PlanDefinitionAction, Reference, TaskInput, TaskOutput,
};
use crate::{codes, resource_struct};

resource_struct!(CarePlan {
    id: Option<String>,
    meta: Option<Meta>,
    extension: Vec<Extension>,
    instantiates_canonical: Vec<String>,
    status: codes::RequestStatus,
    intent: String,
    category: Vec<CodeableConcept>,
    title: Option<String>,
    subject: Reference,
    period: Option<Period>,
    created: Option<DateTime>,
    custodian: Option<Reference>,
    care_team: Vec<Reference>,
    supporting_info: Vec<Reference>,
    activity: Vec<CarePlanActivity>,
    note: Vec<Annotation>,
});

resource_struct!(CareTeam {
    id: Option<String>,
    meta: Option<Meta>,
    participant: Vec<CareTeamParticipant>,
});

resource_struct!(Organization {
    id: Option<String>,
    meta: Option<Meta>,
    extension: Vec<Extension>,
    identifier: Vec<Identifier>,
    active: Option<bool>,
    r#type: Vec<CodeableConcept>,
    name: Option<String>,
    contact: Vec<ExtendedContactDetail>,
});

resource_struct!(Patient {
    id: Option<String>,
    meta: Option<Meta>,
    identifier: Vec<Identifier>,
    active: Option<bool>,
    name: Vec<HumanName>,
});

resource_struct!(PlanDefinition {
    id: Option<String>,
    meta: Option<Meta>,
    contained: Vec<Resource>,
    url: Option<String>,
    version: Option<String>,
    title: Option<String>,
    status: codes::PublicationStatus,
    action: Vec<PlanDefinitionAction>,
});

resource_struct!(Practitioner {
    id: Option<String>,
    meta: Option<Meta>,
    identifier: Vec<Identifier>,
    active: Option<bool>,
    name: Vec<HumanName>,
    telecom: Vec<ContactPoint>,
});

resource_struct!(ActivityDefinition {
    id: Option<String>,
    meta: Option<Meta>,
    extension: Vec<Extension>,
    title: Option<String>,
    status: codes::PublicationStatus,
    kind: Option<String>,
    intent: Option<codes::RequestIntent>,
});

resource_struct!(ServiceRequest {
    id: String,
    meta: Option<Meta>,
    extension: Vec<Extension>,
    based_on: Vec<Reference>,
    status: codes::RequestStatus,
    intent: codes::RequestIntent,
    subject: Reference,
    // occurrence: Option<ServiceRequestOccurrence>,
    authored_on: Option<DateTime>,
    requester: Option<Reference>,
});

resource_struct!(Task {
    id: Option<String>,
    meta: Option<Meta>,
    extension: Vec<Extension>,
    instantiates_canonical: Option<String>,
    based_on: Vec<Reference>,
    status: codes::TaskStatus,
    business_status: Option<CodeableConcept>,
    intent: String,
    r#for: Option<Reference>,
    requested_period: Option<Period>,
    authored_on: Option<DateTime>,
    last_modified: Option<DateTime>,
    requester: Option<Reference>,
    owner: Option<Reference>,
    input: Vec<TaskInput>,
    output: Vec<TaskOutput>,
});

resource_struct!(PractitionerRole {
    id: Option<String>,
    meta: Option<Meta>,
    active: Option<bool>,
    practitioner: Option<Reference>,
    organization: Option<Reference>,
    code: Vec<CodeableConcept>,
    specialty: Vec<CodeableConcept>,
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
