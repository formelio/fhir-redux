use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::date_time::{DateTime, Instant, Time};
use crate::{codes, type_struct};

type_struct!(Meta {
    pub extension: Vec<Extension>,
    pub version_id: Option<String>,
    pub last_updated: Option<Instant>,
    pub source: Option<String>,
    pub tag: Vec<Coding>,
});

type_struct!(Extension {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub url: String,
    #[serde(flatten)]
    pub value: Option<ExtensionValue>,
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValue {
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    #[serde(rename = "valueInteger")]
    Integer(i32),
    #[serde(rename = "valueReference")]
    Reference(Reference),
    #[serde(rename = "valueString")]
    String(String),
    #[serde(rename = "valueUri")]
    Uri(String),
}

type_struct!(Coding {
    pub system: String,
    pub code: String,
    pub display: Option<String>,
});

type_struct!(CodeableConcept {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub coding: Vec<Coding>,
    pub text: Option<String>,
});

type_struct!(Reference {
    pub reference: Option<String>,
    pub display: Option<String>,
});

type_struct!(Period {
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
});

type_struct!(CodeableReference {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub concept: Option<CodeableConcept>,
    pub reference: Option<Reference>,
});

type_struct!(Annotation {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub author_reference: Option<Reference>,
    pub time: Option<DateTime>,
    pub text: String,
});

type_struct!(CareTeamParticipant {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub role: Option<CodeableConcept>,
    pub member: Option<Reference>,
    pub on_behalf_of: Option<Reference>,
    pub coverage_period: Option<Period>,
});

type_struct!(Identifier {
    pub system: String,
    pub value: Option<String>,
});

type_struct!(ExtendedContactDetail {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub purpose: Option<CodeableConcept>,
    pub name: Vec<HumanName>,
    pub telecom: Vec<ContactPoint>,
    pub address: Option<Address>,
    pub organization: Option<Reference>,
    pub period: Option<Period>,
});

type_struct!(HumanName {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub r#use: Option<codes::NameUse>,
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Vec<String>,
    pub prefix: Vec<String>,
    pub suffix: Vec<String>,
    pub period: Option<Period>,
});

type_struct!(ContactPoint {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub system: Option<codes::ContactPointSystem>,
    pub value: Option<String>,
    pub r#use: Option<codes::ContactPointUse>,
    pub rank: Option<NonZeroU32>,
    pub period: Option<Period>,
});

type_struct!(Address {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub r#use: Option<codes::AddressUse>,
    pub r#type: Option<codes::AddressType>,
    pub text: Option<String>,
    pub line: Vec<String>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<Period>,
});

type_struct!(PlanDefinitionAction {
    pub definition_canonical: Option<String>,
});

type_struct!(Dosage {
    pub extension: Vec<Extension>,
    pub timing: Option<Timing>,
    pub dose_and_rate: Vec<DosageDoseAndRate>,
});

type_struct!(Range {
    pub low: Option<Quantity>,
    pub high: Option<Quantity>,
});

type_struct!(DosageDoseAndRate { pub dose_quantity: Quantity });

type_struct!(Quantity {
    pub value: Option<f64>,
});

type_struct!(Timing {
    pub repeat: Option<TimingRepeat>,
});

type_struct!(TimingRepeat {
    pub count: Option<NonZeroU32>,
    pub time_of_day: Vec<Time>,
});

type_struct!(TaskInput {
    pub r#type: CodeableConcept,
    #[serde(flatten)]
    pub value: TaskInputValue
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskInputValue {
    #[serde(rename = "valueAnnotation")]
    Annotation(Annotation),
    #[serde(rename = "valueBoolean")]
    Boolean(bool),
    #[serde(rename = "valueCoding")]
    Coding(Coding),
    #[serde(rename = "valueDosage")]
    Dosage(Dosage),
    #[serde(rename = "valueRange")]
    Range(Range),
    #[serde(rename = "valueString")]
    String(String),
}

type_struct!(TaskOutput { pub r#type: CodeableConcept, pub value_reference: Reference });

type_struct!(QuestionnaireResponseItem {
    pub link_id: String,
    pub text: Option<String>,
    pub answer: Vec<QuestionnaireResponseItemAnswer>,
});

type_struct!(QuestionnaireResponseItemAnswer {
    pub extension: Vec<Extension>,
    #[serde(rename = "valueString")]
    pub value: String,
});
