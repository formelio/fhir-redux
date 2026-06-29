use crate::r5::types;
use crate::resources::{
    ActivityDefinition, CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, PractitionerRole,
    ServiceRequest, Task,
};
use crate::{codes, resource, type_struct};

type_struct!(Bundle {
    pub id: String,
    pub r#type: codes::BundleType,
    pub link: Vec<types::BundleLink>,
    pub entry: Vec<types::BundleEntry>,
});

resource!([
    ActivityDefinition,
    Bundle,
    CarePlan,
    CareTeam,
    Organization,
    Patient,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    ServiceRequest,
    Task,
]);
