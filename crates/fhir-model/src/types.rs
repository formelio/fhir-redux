use std::num::NonZeroU32;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::codes;
use crate::date_time::{DateTime, Instant, Time};

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub version_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub last_updated: Option<Instant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<String>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // #[builder(default)]
    // pub profile: Vec<String>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // #[builder(default)]
    // pub security: Vec<Coding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub tag: Vec<Coding>,
}

// R4 & R5 compatible (note `value` has commented out types that are R5 only: integer64, CodeableReference, RatioRange, Availability, ExtendedContactDetail, Meta)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[serde(flatten)]
    pub value: Option<ExtensionValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionValue {
    //     #[serde(rename = "valueBase64Binary")]
    //     Base64Binary(Base64Binary),
    //     #[serde(rename = "valueBoolean")]
    //     Boolean(bool),
    //     #[serde(rename = "valueCanonical")]
    //     Canonical(String),
    //     #[serde(rename = "valueCode")]
    //     Code(String),
    //     #[serde(rename = "valueDate")]
    //     Date(Date),
    //     #[serde(rename = "valueDateTime")]
    //     DateTime(DateTime),
    //     #[serde(rename = "valueDecimal")]
    //     Decimal(f64),
    //     #[serde(rename = "valueId")]
    //     Id(String),
    //     #[serde(rename = "valueInstant")]
    //     Instant(Instant),
    #[serde(rename = "valueInteger")]
    Integer(i32),
    //     #[serde(rename = "valueInteger64")]
    //     Integer64(Integer64),
    //     #[serde(rename = "valueMarkdown")]
    //     Markdown(String),
    //     #[serde(rename = "valueOid")]
    //     Oid(String),
    //     #[serde(rename = "valuePositiveInt")]
    //     PositiveInt(NonZeroU32),
    //     #[serde(rename = "valueString")]
    //     String(String),
    //     #[serde(rename = "valueTime")]
    //     Time(Time),
    //     #[serde(rename = "valueUnsignedInt")]
    //     UnsignedInt(u32),
    #[serde(rename = "valueUri")]
    Uri(String),
    //     #[serde(rename = "valueUrl")]
    //     Url(String),
    //     #[serde(rename = "valueUuid")]
    //     Uuid(String),
    //     #[serde(rename = "valueAddress")]
    //     Address(Address),
    //     #[serde(rename = "valueAge")]
    //     Age(Age),
    //     #[serde(rename = "valueAnnotation")]
    //     Annotation(Annotation),
    //     #[serde(rename = "valueAttachment")]
    //     Attachment(Attachment),
    //     #[serde(rename = "valueCodeableConcept")]
    //     CodeableConcept(CodeableConcept),
    //     #[serde(rename = "valueCodeableReference")]
    //     CodeableReference(CodeableReference),
    //     #[serde(rename = "valueCoding")]
    //     Coding(Coding),
    //     #[serde(rename = "valueContactPoint")]
    //     ContactPoint(ContactPoint),
    //     #[serde(rename = "valueCount")]
    //     Count(Count),
    //     #[serde(rename = "valueDistance")]
    //     Distance(Distance),
    //     #[serde(rename = "valueDuration")]
    //     Duration(Duration),
    //     #[serde(rename = "valueHumanName")]
    //     HumanName(HumanName),
    //     #[serde(rename = "valueIdentifier")]
    //     Identifier(Identifier),
    //     #[serde(rename = "valueMoney")]
    //     Money(Money),
    //     #[serde(rename = "valuePeriod")]
    //     Period(Period),
    //     #[serde(rename = "valueQuantity")]
    //     Quantity(Quantity),
    //     #[serde(rename = "valueRange")]
    //     Range(Range),
    //     #[serde(rename = "valueRatio")]
    //     Ratio(Ratio),
    //     #[serde(rename = "valueRatioRange")]
    //     RatioRange(RatioRange),
    //     #[serde(rename = "valueReference")]
    //     Reference(Reference),
    //     #[serde(rename = "valueSampledData")]
    //     SampledData(SampledData),
    //     #[serde(rename = "valueSignature")]
    //     Signature(Signature),
    //     #[serde(rename = "valueTiming")]
    //     Timing(Timing),
    //     #[serde(rename = "valueContactDetail")]
    //     ContactDetail(ContactDetail),
    //     #[serde(rename = "valueDataRequirement")]
    //     DataRequirement(DataRequirement),
    //     #[serde(rename = "valueExpression")]
    //     Expression(Expression),
    //     #[serde(rename = "valueParameterDefinition")]
    //     ParameterDefinition(ParameterDefinition),
    //     #[serde(rename = "valueRelatedArtifact")]
    //     RelatedArtifact(RelatedArtifact),
    //     #[serde(rename = "valueTriggerDefinition")]
    //     TriggerDefinition(TriggerDefinition),
    //     #[serde(rename = "valueUsageContext")]
    //     UsageContext(UsageContext),
    //     #[serde(rename = "valueAvailability")]
    //     Availability(Availability),
    //     #[serde(rename = "valueExtendedContactDetail")]
    //     ExtendedContactDetail(ExtendedContactDetail),
    //     #[serde(rename = "valueDosage")]
    //     Dosage(Dosage),
    //     #[serde(rename = "valueMeta")]
    //     Meta(Meta),
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
    pub system: String,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub display: Option<String>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeableConcept {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub coding: Vec<Coding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<String>,
}

// R4 & R5 compatible (note commented `type` field might be incompatible)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub id: Option<String>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // #[builder(default)]
    // pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reference: Option<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub r#type: Option<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub identifier: Option<Identifier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub display: Option<String>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub id: Option<String>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // #[builder(default)]
    // pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub start: Option<DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub end: Option<DateTime>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CodeableReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub concept: Option<CodeableConcept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reference: Option<Reference>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub author_reference: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time: Option<DateTime>,
    pub text: String,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CareTeamParticipant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub modifier_extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub role: Option<CodeableConcept>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub member: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on_behalf_of: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub coverage_period: Option<Period>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    #[builder(default)]
    pub system: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedContactDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub purpose: Option<CodeableConcept>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub name: Vec<HumanName>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub telecom: Vec<ContactPoint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub organization: Option<Reference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub period: Option<Period>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r#use: Option<codes::NameUse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub given: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub prefix: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub suffix: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub period: Option<Period>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ContactPoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub system: Option<codes::ContactPointSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r#use: Option<codes::ContactPointUse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rank: Option<NonZeroU32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub period: Option<Period>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r#use: Option<codes::AddressUse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r#type: Option<codes::AddressType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub line: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub district: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub period: Option<Period>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub definition_canonical: Option<String>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Dosage {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub timing: Option<Timing>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub dose_and_rate: Vec<DosageDoseAndRate>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub low: Option<Quantity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub high: Option<Quantity>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DosageDoseAndRate {
    pub dose_quantity: Quantity,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub id: Option<String>,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // #[builder(default)]
    // pub extension: Vec<Extension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub comparator: Option<codes::QuantityComparator>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub unit: Option<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub system: Option<String>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // #[builder(default)]
    // pub code: Option<String>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<TimingRepeat>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TimingRepeat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<NonZeroU32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[builder(default)]
    pub time_of_day: Vec<Time>,
}

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    pub r#type: CodeableConcept,
    #[serde(flatten)]
    pub value: TaskInputValue,
}

// R4 & R5 compatible
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

// R4 & R5 compatible
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct TaskOutput {
    pub r#type: CodeableConcept,
    pub value_reference: Reference,
}
