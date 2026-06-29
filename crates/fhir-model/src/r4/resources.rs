use crate::date_time::DateTime;
use crate::r4::types::BundleEntry;
use crate::resources::{
    ActivityDefinition, CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, PractitionerRole,
    ResourceType, ServiceRequest,
};
use crate::types::{
    // Annotation, CareTeamParticipant, PlanDefinitionAction, HumanName, ContactPoint, ExtendedContactDetail, TaskInput, TaskOutput,
    CodeableConcept,
    CodeableConceptBuilder,
    Coding,
    CodingBuilder,
    Extension,
    Identifier,
    Meta,
    Period,
    Reference,
};
use crate::{codes, resource, resource_struct};

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct CarePlan {
//     #[serde(default = "CarePlan::resource_type")]
//     #[builder(default = "ResourceType::CarePlan", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub extension: Vec<Extension>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub instantiates_canonical: Vec<String>,
//     pub status: codes::RequestStatus,
//     pub intent: String,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub category: Vec<CodeableConcept>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub title: Option<String>,
//     pub subject: Reference,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub period: Option<Period>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub created: Option<DateTime>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub custodian: Option<Reference>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub care_team: Vec<Reference>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub supporting_info: Vec<Reference>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub activity: Vec<CarePlanActivity>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub note: Vec<Annotation>,
// }
//
// impl CarePlan {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::CarePlan
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct CareTeam {
//     /// Type of this FHIR resource.
//     #[serde(default = "CareTeam::resource_type")]
//     #[builder(default = "ResourceType::CareTeam", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub participant: Vec<CareTeamParticipant>,
// }
//
// impl CareTeam {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::CareTeam
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct Organization {
//     #[serde(default = "Organization::resource_type")]
//     #[builder(default = "ResourceType::Organization", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub extension: Vec<Extension>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub identifier: Vec<Identifier>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub active: Option<bool>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub r#type: Vec<CodeableConcept>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub name: Option<String>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub contact: Vec<ExtendedContactDetail>,
// }
//
// impl Organization {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::Organization
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct Patient {
//     #[serde(default = "Patient::resource_type")]
//     #[builder(default = "ResourceType::Patient", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub identifier: Vec<Identifier>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub active: Option<bool>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub name: Vec<HumanName>,
// }
//
// impl Patient {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::Patient
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct PlanDefinition {
//     #[serde(default = "PlanDefinition::resource_type")]
//     #[builder(default = "ResourceType::PlanDefinition", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub contained: Vec<Resource>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub url: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub version: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub title: Option<String>,
//     pub status: codes::PublicationStatus,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub action: Vec<PlanDefinitionAction>,
// }
//
// impl PlanDefinition {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::PlanDefinition
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct Practitioner {
//     #[serde(default = "Practitioner::resource_type")]
//     #[builder(default = "ResourceType::Practitioner", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub identifier: Vec<Identifier>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub active: Option<bool>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub name: Vec<HumanName>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub telecom: Vec<ContactPoint>,
// }
//
// impl Practitioner {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::Practitioner
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct ActivityDefinition {
//     #[serde(default = "ActivityDefinition::resource_type")]
//     #[builder(default = "ResourceType::ActivityDefinition", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub extension: Vec<Extension>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub title: Option<String>,
//     pub status: codes::PublicationStatus,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub kind: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub intent: Option<codes::RequestIntent>,
// }
//
// impl ActivityDefinition {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::ActivityDefinition
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct ServiceRequest {
//     #[serde(default = "ServiceRequest::resource_type")]
//     #[builder(default = "ResourceType::ServiceRequest", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub extension: Vec<Extension>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub based_on: Vec<Reference>,
//     pub status: codes::RequestStatus,
//     pub intent: codes::RequestIntent,
//     pub subject: Reference,
//     // #[serde(default, skip_serializing_if = "Option::is_none")]
//     // #[builder(default)]
//     // #[serde(flatten)]
//     // pub occurrence: Option<ServiceRequestOccurrence>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub authored_on: Option<DateTime>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub requester: Option<Reference>,
// }
//
// impl ServiceRequest {
//     /// Get the resource type for this FHIR resource.
//     #[must_use]
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::ServiceRequest
//     }
// }
//
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
// #[serde(rename_all = "camelCase")]
// pub struct PractitionerRole {
//     #[serde(default = "PractitionerRole::resource_type")]
//     #[builder(default = "ResourceType::PractitionerRole", setter(skip))]
//     resource_type: ResourceType,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub id: Option<String>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub meta: Option<Meta>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub active: Option<bool>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub practitioner: Option<Reference>,
//     #[serde(default, skip_serializing_if = "Option::is_none")]
//     #[builder(default)]
//     pub organization: Option<Reference>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub code: Vec<CodeableConcept>,
//     #[serde(default, skip_serializing_if = "Vec::is_empty")]
//     #[builder(default)]
//     pub specialty: Vec<CodeableConcept>,
// }
//
// impl PractitionerRole {
//     pub const fn resource_type() -> ResourceType {
//         ResourceType::PractitionerRole
//     }
// }

// Changes from r5
// - removed requested_period
// - added fields below
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
    pub authored_on: Option<DateTime>,
    pub last_updated: Option<DateTime>,
    pub requester: Option<Reference>,
    pub owner: Option<Reference>,

    pub identifier: Vec<Identifier>,
    pub execution_period: Option<Period>,
    pub focus: Option<Reference>,
    pub description: Option<String>,
});

resource_struct!(Endpoint {
    pub status: codes::EndpointStatus,
    #[serde(default = "Endpoint::connection_type")]
    #[builder(default = "Endpoint::connection_type()")]
    pub connection_type: Coding,
    #[serde(default = "Endpoint::payload_type")]
    #[builder(default = "Endpoint::payload_type()")]
    @nodefault pub payload_type: Vec<CodeableConcept>,
    pub address: String,
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

resource_struct!(Bundle {
    pub id: String,
    pub r#type: codes::BundleType,
    pub entry: Vec<BundleEntry>,
});

resource!([
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
]);
