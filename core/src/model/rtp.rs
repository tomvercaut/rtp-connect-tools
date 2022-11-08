use crate::model::{ExtendedPlan, Field, Plan, Prescription, Simulation, SiteSetup};

pub struct Rtp {
    pub plan: Plan,
    pub extended_plan: ExtendedPlan,
    pub prescription: Prescription,
    pub site_setup: SiteSetup,
    pub simulation: Simulation,
    pub fields: Vec<Field>,
}

impl std::default::Default for Rtp {
    fn default() -> Self {
        Self {
            plan: Default::default(),
            extended_plan: Default::default(),
            prescription: Default::default(),
            site_setup: Default::default(),
            simulation: Default::default(),
            fields: vec![],
        }
    }
}

impl Rtp {
    pub fn new() -> Rtp {
        Self::default()
    }
}