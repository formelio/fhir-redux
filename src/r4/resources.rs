use crate::date_time::DateTime;
use crate::r4::types::BundleEntry;
use crate::resources::{
    ActivityDefinition, CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, PractitionerRole,
    ServiceRequest,
};
use crate::types::{
    CodeableConcept, CodeableConceptBuilder, Coding, CodingBuilder, Extension, Identifier, Meta, Period, Reference,
};
use crate::{codes, resource, type_struct};

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
    pub last_updated: Option<DateTime>,
    pub requester: Option<Reference>,
    pub owner: Option<Reference>,

    pub identifier: Vec<Identifier>,
    pub execution_period: Option<Period>,
    pub focus: Option<Reference>,
    pub description: Option<String>,
});

type_struct!(Endpoint {
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

type_struct!(Bundle {
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
