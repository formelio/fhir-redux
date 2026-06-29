use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::date_time::{DateTime, Instant, Time};
use crate::{codes, type_struct};

// R4 & R5 compatible
type_struct!(Meta {
    // id: Option<String>,
    pub extension: Vec<Extension>,
    pub version_id: Option<String>,
    pub last_updated: Option<Instant>,
    pub source: Option<String>,
    // profile: Vec<String>,
    // security: Vec<Coding>,
    pub tag: Vec<Coding>,
});

// R4 & R5 compatible (note `value` has commented out types that are R5 only: integer64, CodeableReference, RatioRange, Availability, ExtendedContactDetail, Meta)
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
type_struct!(Coding {
    pub system: String,
    pub code: String,
    pub display: Option<String>,
});

// R4 & R5 compatible
type_struct!(CodeableConcept {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub coding: Vec<Coding>,
    pub text: Option<String>,
});

// R4 & R5 compatible (note commented `type` field might be incompatible)
type_struct!(Reference {
    // id: Option<String>,
    // extension: Vec<Extension>,
    pub reference: Option<String>,
    // r#type: Option<String>,
    // identifier: Option<Identifier>,
    pub display: Option<String>,
});

// R4 & R5 compatible
type_struct!(Period {
    // id: Option<String>,
    // extension: Vec<Extension>,
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,
});

// R4 & R5 compatible
type_struct!(CodeableReference {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub concept: Option<CodeableConcept>,
    pub reference: Option<Reference>,
});

// R4 & R5 compatible
type_struct!(Annotation {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub author_reference: Option<Reference>,
    pub time: Option<DateTime>,
    pub text: String,
});

// R4 & R5 compatible
type_struct!(CareTeamParticipant {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub modifier_extension: Vec<Extension>,
    pub role: Option<CodeableConcept>,
    pub member: Option<Reference>,
    pub on_behalf_of: Option<Reference>,
    pub coverage_period: Option<Period>,
});

// R4 & R5 compatible
type_struct!(Identifier {
    pub system: String,
    pub value: Option<String>,
});

// R4 & R5 compatible
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

// R4 & R5 compatible
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

// R4 & R5 compatible
type_struct!(ContactPoint {
    pub id: Option<String>,
    pub extension: Vec<Extension>,
    pub system: Option<codes::ContactPointSystem>,
    pub value: Option<String>,
    pub r#use: Option<codes::ContactPointUse>,
    pub rank: Option<NonZeroU32>,
    pub period: Option<Period>,
});

// R4 & R5 compatible
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

// R4 & R5 compatible
type_struct!(PlanDefinitionAction {
    pub definition_canonical: Option<String>,
});

// R4 & R5 compatible
type_struct!(Dosage {
    pub extension: Vec<Extension>,
    pub timing: Option<Timing>,
    pub dose_and_rate: Vec<DosageDoseAndRate>,
});

// R4 & R5 compatible
type_struct!(Range {
    pub low: Option<Quantity>,
    pub high: Option<Quantity>,
});

// R4 & R5 compatible
type_struct!(DosageDoseAndRate { pub dose_quantity: Quantity });

// R4 & R5 compatible
type_struct!(Quantity {
    // id: Option<String>,
    // extension: Vec<Extension>,
    pub value: Option<f64>,
    // comparator: Option<codes::QuantityComparator>,
    // unit: Option<String>,
    // system: Option<String>,
    // code: Option<String>,
});

// R4 & R5 compatible
type_struct!(Timing {
    pub repeat: Option<TimingRepeat>,
});

// R4 & R5 compatible
type_struct!(TimingRepeat {
    pub count: Option<NonZeroU32>,
    pub time_of_day: Vec<Time>,
});

// R4 & R5 compatible
type_struct!(TaskInput {
    pub r#type: CodeableConcept,
    #[serde(flatten)]
    pub value: TaskInputValue
});

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
type_struct!(TaskOutput { pub r#type: CodeableConcept, pub value_reference: Reference });
