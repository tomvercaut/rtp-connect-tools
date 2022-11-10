use crate::model::{ControlPoint, DocumentBasedTreatmentField, DoseAction, DoseTracking, ExtendedField, ExtendedPlan, Field, MlcShape, MultiLeafCollimator, Plan, Prescription, Simulation, SiteSetup};

pub struct Rtp {
    pub plan: Plan,
    pub extended_plan: ExtendedPlan,
    pub prescription: Prescription,
    pub site_setup: SiteSetup,
    pub simulation: Simulation,
    pub fields: Vec<Field>,
    pub extended_fields: Vec<ExtendedField>,
    pub document_based_treatment_fields: Vec<DocumentBasedTreatmentField>,
    pub mlcs: Vec<MultiLeafCollimator>,
    pub control_points: Vec<ControlPoint>,
    pub mlc_shapes: Vec<MlcShape>,
    pub dose_trackings: Vec<DoseTracking>,
    pub dose_actions: Vec<DoseAction>,
}

impl Default for Rtp {
    fn default() -> Self {
        Self {
            plan: Default::default(),
            extended_plan: Default::default(),
            prescription: Default::default(),
            site_setup: Default::default(),
            simulation: Default::default(),
            fields: vec![],
            extended_fields: vec![],
            document_based_treatment_fields: vec![],
            mlcs: vec![],
            control_points: vec![],
            mlc_shapes: vec![],
            dose_trackings: vec![],
            dose_actions: vec![],
        }
    }
}

impl Rtp {
    pub fn new() -> Rtp {
        Self::default()
    }
}