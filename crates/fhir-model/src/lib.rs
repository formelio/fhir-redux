#![recursion_limit = "1024"]

pub mod codes;
pub mod date_time;
pub mod resources;
pub mod types;

#[cfg(feature = "r4")]
pub mod r4;
#[cfg(feature = "r5")]
pub mod r5;

#[macro_export]
macro_rules! resource {
    ([$($resource:ident),*$(,)?]) => {
        #[derive(serde::Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "resourceType")]
        pub enum Resource {
            $(
            $resource($resource),
            )*
        }

        impl serde::Serialize for Resource {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                    Resource::$resource(inner) => inner.serialize(serializer),
                    )*
                }
            }
        }

        impl Resource {
            pub const fn resource_type(&self) -> ResourceType {
                match self {
                    $(
                    Self::$resource(_) => ResourceType::$resource,
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! resource_struct {
    (@ $resource:ident { } -> ($($fields:tt)*)) => {
        paste::paste! {
            #[derive(serde::Serialize, serde::Deserialize, derive_builder::Builder, Debug, Clone, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct $resource {
                #[serde(default = "" $resource "::resource_type" "")]
                #[builder(default = "" "ResourceType::" $resource "", setter(skip))]
                resource_type: ResourceType,

                $($fields)*
            }

            impl $resource {
                pub const fn resource_type() -> ResourceType {
                    ResourceType::$resource
                }
            }
        }
    };

    ( @ $resource:ident { $key:ident: Option<$value:ty> $(, $($parameters:tt)*)? } -> ($($processed:tt)*) ) => (
        resource_struct!(@ $resource { $($($parameters)*)? } -> (
            $($processed)*
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[builder(default)]
            pub $key: Option<$value>,
        ));
    );

    ( @ $resource:ident { $key:ident: Vec<$value:ty> $(, $($parameters:tt)*)? } -> ($($processed:tt)*) ) => (
        resource_struct!(@ $resource { $($($parameters)*)? } -> (
            $($processed)*
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            #[builder(default)]
            pub $key: Vec<$value>,
        ));
    );

    ( @ $resource:ident { $key:ident: Vec<$value:ty> $(, $($parameters:tt)*)? } -> ($($processed:tt)*) ) => (
        resource_struct!(@ $resource { $($($parameters)*)? } -> (
            $($processed)*
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            #[builder(default)]
            pub $key: Vec<$value>,
        ));
    );

    ( @ $resource:ident { $(#[$attribute:meta])* $key:ident: $value:ty $(, $($parameters:tt)*)? } -> ($($processed:tt)*) ) => (
        resource_struct!(@ $resource { $($($parameters)*)? } -> (
            $($processed)*
            $(#[$attribute])*
            pub $key: $value,
        ));
    );

    ( $resource:ident { $($body:tt)* } ) => (
        resource_struct!(@ $resource { $($body)* } -> ());
    );
}
