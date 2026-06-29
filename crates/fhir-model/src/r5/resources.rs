use crate::r5::types;
use crate::resources::{
    ActivityDefinition, CarePlan, CareTeam, Organization, Patient, PlanDefinition, Practitioner, PractitionerRole,
    ResourceType, ServiceRequest, Task,
};
use crate::{codes, resource, resource_struct};

resource_struct!(Bundle {
    pub id: String,
    pub r#type: codes::BundleType,
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
