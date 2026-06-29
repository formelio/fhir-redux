# FHIR Redux

This model set is based on the HL7 FHIR specifications and the work of FlixCoder, [fhir_sdk](https://github.com/FlixCoder/fhir-sdk).

It is a lean set of resources, tailored to Formelio's needs. Not all resources or all fields are defined.

🔥🦆

## Adding resources, types and codes

FHIR Redux is built up from a 3-level system:

1. **Resources**: First-class citizens like `Bundle`, `QuestionnaireResponse` and `Patient`.
2. **Types**: Custom structs used in resources and types, like `Extension`, `BundleLink` and `TaskInput`.
3. **Codes**: Custom enums used in resources and types, like `RequestStatus`, `TaskStatus` and `SearchEntryMode`.

To make things as simple as possible, two macros are available to create resources and types, and codes. These should allow all the necessary configuration without any of the bloat.

### Adding Types

Adding resources requires the same steps as adding types, with one extra step afterwards. Therefore, we start with adding types.

1. Determine the version of the type you are adding.
    1. Most types can live in the generic list of types (`/src/types.rs`, or `/src/resources.rs` for resources) as the slimmed-down version we use is identical between `stu3`, `r4` and `r5`.
    2. Sometimes, types can differ, and should be placed in their respective `/src/{version}/types.rs` file (or `/src/{version}/resources.rs` for resources).
2. To create a type, the `type_struct!` macro is provided to make life easy:
    ```
    type_struct!(TimingRepeat {
        pub r#type: CodeableConcept,
        pub count: Option<NonZeroU32>,
        pub time_of_day: Vec<Time>,
    });
    ```
    What you will find is that the macro closely represents an actual `struct` definition where the visibility token (`pub`), identifier (e.g. `count`) and type are all parsed as you'd expect. The macro automatically adds all necessary attributes to the struct, allowing you to focus and keep a compact overview of your structs. `Option`s and `Vec`s are automatically provided with a `default` flag in both serde and strum, but this can be disabled by prefixing a line with `@nodefault`. Note that all other fields (e.g. `r#type`) that are not `Option` or `Vec` will not receive a default at all.
    Sometimes, extra field attributes are required, or you want to set a custom field attribute default (done in conjunction with `@nodefault`). This is possible as well:
    ```
    type_struct!(SomeType {
        #[serde(default = "coding")]
        #[builder(default = "coding()")]
        pub coding: Coding,

        #[serde(default = "vec_coding")]
        #[builder(default = "vec_coding()")]
        @nodefault pub vec_coding: Vec<Coding>,

        #[serde(flatten)]
        pub value: SomeTypeValue
    });

    fn coding() -> Coding { ... }
    fn vec_coding() -> Vec<Coding> { ... }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum SomeTypeValue {
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
    ```
3. That's it. Simple types usually don't need tweaking with `@nodefault` and custom field attributes, but the option is there when you do happen to need it.

### Adding Resources

Adding resources requires the exact same steps as types, with one added step at the end.

1. Follow the steps defined under Adding Types.
2. Add your new resource to the Resource enum in the relevant version(s): `/src/{version}/resources.rs`. Make sure to import the resource if you added it to the generic resources.

### Adding Codes

Adding codes is also made simple by providing a macro to do the heavy lifting. All current codes are generic and reused between versions. If this happens to change at some point, the same hierarchy can be introduced as `/src/{version}/codes.rs`, but for now it all lives in `/src/codes.rs`.

Making a code is as simple as can be:

```
code!(LinkRelationTypes, [Next, Prev, _Self = "self"]);
```

The macro automatically creates an enum for you with the given options in the list. To avoid issues with keywords, fields can be renamed with an optional `= "{serialised-name}"` after it, though this needs only be used in rare cases such as when needing to parse `"self"`.
