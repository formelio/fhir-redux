use crate::types::{CodeableConcept, Coding};

macro_rules! code {
    ($resource:ident, [$($value:ident),*$(,)?]) => {
        #[derive(
            serde::Deserialize,
            serde::Serialize,
            PartialEq,
            Eq,
            Clone,
            Hash,
            Copy,
            strum::AsRefStr,
            strum::EnumString,
            Debug,
            strum::Display,
        )]
        #[strum(serialize_all = "kebab-case")]
        #[serde(rename_all = "kebab-case")]
        pub enum $resource {
            $(
            $value,
            )*
        }

        impl From<$resource> for Coding {
            fn from(code: $resource) -> Self {
                Coding {
                    system: "http://hl7.org/fhir/ValueSet/request-status".to_owned(),
                    code: code.as_ref().to_owned(),
                    display: Some(format!("{code}")),
                }
            }
        }

        impl From<$resource> for CodeableConcept {
            fn from(code: $resource) -> Self {
                let text = format!("{code}");
                let coding = Coding::from(code);
                CodeableConcept { coding: vec![coding], text: Some(text), id: None, extension: Vec::new() }
            }
        }
    };
}

code!(RequestStatus, [Active, Completed, Draft, EnteredInError, OnHold, Revoked, Unknown]);
code!(NameUse, [Anonymous, Maiden, Nickname, Official, Old, Temp, Usual]);

code!(ContactPointSystem, [Email, Fax, Other, Pager, Phone, Sms, Url]);
code!(ContactPointUse, [Home, Mobile, Old, Temp, Work]);
code!(AddressUse, [Billing, Home, Old, Temp, Work]);
code!(AddressType, [Both, Physical, Postal]);
code!(PublicationStatus, [Active, Draft, Retired, Unknown]);
code!(
    RequestIntent,
    [Directive, FillerOrder, InstanceOrder, Option, Order, OriginalOrder, Plan, Proposal, ReflexOrder]
);
code!(
    TaskStatus,
    [
        Accepted,
        Cancelled,
        Completed,
        Draft,
        EnteredInError,
        Failed,
        InProgress,
        OnHold,
        Ready,
        Received,
        Rejected,
        Requested,
    ]
);
code!(EndpointStatus, [Active, EnteredInError, Error, Off, Suspended]);
code!(SearchEntryMode, [Include, Match, Outcome]);

code!(
    BundleType,
    [Batch, BatchResponse, Collection, Document, History, Message, Searchset, Transaction, TransactionResponse]
);
