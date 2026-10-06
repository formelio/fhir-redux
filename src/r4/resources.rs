use crate::date_time::{Date, DateTime};
use crate::r4::types::{
    BundleEntry, BundleLink, ConsentPolicy, ConsentProvision, DeviceDeviceName, DeviceUdiCarrier, EncounterParticipant,
    ProcedurePerformer,
};
use crate::resources::{
    CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, QuestionnaireResponse, ServiceRequest,
};
use crate::types::{
    Address, Annotation, Attachment, CodeableConcept, CodeableConceptBuilder, Coding, CodingBuilder, ContactPoint,
    Extension, HumanName, Identifier, Meta, Period, Reference,
};
use crate::{codes, resource, type_struct};

type_struct!(ActivityDefinition {
    pub id: String,
    pub meta: Meta,
    pub extension: Vec<Extension>,
    pub title: String,
    pub description: String,
    pub status: codes::PublicationStatus,
    pub url: String,
});

type_struct!(Task {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub instantiates_canonical: Option<String>,
    pub based_on: Vec<Reference>,
    pub status: codes::TaskStatus,
    pub business_status: Option<CodeableConcept>,
    pub intent: String,
    pub r#for: Option<Reference>,
    pub authored_on: Option<DateTime>,
    pub requester: Option<Reference>,
    pub owner: Option<Reference>,

    pub identifier: Vec<Identifier>,
    pub execution_period: Option<Period>,
    pub focus: Option<Reference>,
    pub description: Option<String>,
});

type_struct!(Endpoint {
    pub id: String,
    pub status: codes::EndpointStatus,
    #[serde(default = "Endpoint::connection_type")]
    #[builder(default = "Endpoint::connection_type()")]
    pub connection_type: Coding,
    #[serde(default = "Endpoint::payload_type")]
    #[builder(default = "Endpoint::payload_type()")]
    @nodefault pub payload_type: Vec<CodeableConcept>,
    pub address: String,
    pub meta: Meta,
});

impl Endpoint {
    pub fn connection_type() -> Coding {
        CodingBuilder::default()
            .system("http://terminology.hl7.org/CodeSystem/endpoint-connection-type".to_string())
            .code("hl7-fhir-rest".to_string())
            .build()
            .expect("Builder should succeed")
    }

    pub fn payload_type() -> Vec<CodeableConcept> {
        vec![
            CodeableConceptBuilder::default()
                .coding(vec![
                    CodingBuilder::default()
                        .system("http://terminology.hl7.org/CodeSystem/endpoint-payload-type".to_string())
                        .code("any".to_string())
                        .build()
                        .expect("Builder should succeed"),
                ])
                .build()
                .expect("Builder should succeed"),
        ]
    }
}

type_struct!(Bundle {
    pub id: String,
    pub meta: Option<Meta>,
    pub r#type: codes::BundleType,
    pub total: Option<u32>,
    pub link: Vec<BundleLink>,
    pub entry: Vec<BundleEntry>,
});

type_struct!(PractitionerRole {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub contained: Vec<Resource>,
    pub practitioner: Option<Reference>,
    pub organization: Option<Reference>,
    pub specialty: Vec<CodeableConcept>,
});

type_struct!(Consent {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub status: codes::ConsentState,
    pub scope: CodeableConcept,
    pub category: Vec<CodeableConcept>,
    pub patient: Option<Reference>,
    pub date_time: Option<DateTime>,
    pub source_attachment: Option<Attachment>,
    pub policy: Vec<ConsentPolicy>,
    pub provision: Option<ConsentProvision>,
});

type_struct!(Device {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub identifier: Vec<Identifier>,
    pub udi_carrier: Vec<DeviceUdiCarrier>,
    pub r#type: Option<CodeableConcept>,
    pub device_name: Vec<DeviceDeviceName>,
});

type_struct!(DeviceUseStatement {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::DeviceUseStatementStatus,
    pub subject: Reference,
    pub timing_period: Option<Period>,
    pub source: Option<Reference>,
    pub device: Reference,
    pub reason_reference: Vec<Reference>,
    pub body_site: Option<CodeableConcept>,
    pub location: Option<Reference>,
    pub note: Vec<Annotation>,
});

type_struct!(Encounter {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::EncounterStatus,
    pub class: Coding,
    pub r#type: Vec<CodeableConcept>,
    pub subject: Option<Reference>,
    pub participant: Vec<EncounterParticipant>,
    pub period: Option<Period>,
    pub reason_code: Vec<CodeableConcept>,
    pub reason_reference: Vec<Reference>,
});

type_struct!(Goal {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub lifecycle_status: codes::GoalLifecycleStatus,
    pub category: Vec<CodeableConcept>,
    pub description: CodeableConcept,
    pub subject: Reference,
    pub start_date: Option<Date>,
    pub note: Vec<Annotation>,
});

type_struct!(Observation {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::ObservationStatus,
    pub code: CodeableConcept,
    pub subject: Option<Reference>,
    pub effective_date_time: Option<DateTime>,
    pub value_boolean: Option<bool>,
    pub value_codeable_concept: Option<CodeableConcept>,
    pub value_string: Option<String>,
    pub data_absent_reason: Option<CodeableConcept>,
    pub note: Vec<Annotation>,
    pub method: Option<CodeableConcept>,
});

type_struct!(Procedure {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::EventStatus,
    pub code: Option<CodeableConcept>,
    pub subject: Reference,
    pub encounter: Option<Reference>,
    pub performed_date_time: Option<DateTime>,
    pub performer: Vec<ProcedurePerformer>,
});

type_struct!(RelatedPerson {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub patient: Reference,
    pub relationship: Vec<CodeableConcept>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub address: Vec<Address>,
});

type_struct!(CommunicationRequest {
    pub id: Option<String>,
    pub meta: Option<Meta>,
    pub extension: Vec<Extension>,
    pub status: codes::RequestStatus,
    pub category: Vec<CodeableConcept>,
    pub subject: Option<Reference>,
    pub encounter: Option<Reference>,
    pub authored_on: Option<DateTime>,
    pub requester: Option<Reference>,
    pub recipient: Vec<Reference>,
    pub sender: Option<Reference>,
    pub reason_code: Vec<CodeableConcept>,
});

resource!([
    ActivityDefinition,
    Bundle,
    CarePlan,
    CareTeam,
    CommunicationRequest,
    Consent,
    Device,
    DeviceUseStatement,
    Encounter,
    Endpoint,
    Goal,
    Observation,
    Organization,
    Patient,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    Procedure,
    QuestionnaireResponse,
    RelatedPerson,
    ServiceRequest,
    Task,
]);
