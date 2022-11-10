#[derive(Debug, Clone)]
pub struct SiteSetup {
    pub rx_site_name: String,
    pub patient_orientation: String,
    pub treatment_machine: String,
    pub tolerance_table: String,
    pub isocenter_position_x: Option<f64>,
    pub isocenter_position_y: Option<f64>,
    pub isocenter_position_z: Option<f64>,
    pub structure_set_uid: String,
    pub frame_of_reference_uid: String,
    pub couch_vertical: Option<f64>,
    pub couch_lateral: Option<f64>,
    pub couch_longitudinal: Option<f64>,
    pub couch_angle: Option<f64>,
    pub couch_pedestal: Option<f64>,
    pub table_top_vertical_displacement: Option<f64>,
    pub table_top_longitudinal_displacement: Option<f64>,
    pub table_top_lateral_displacement: Option<f64>,
    pub mrl_coil_name: String,
    pub mrl_coil_index: Option<u32>,
    pub couch_reference: String,
    pub couch_reference_index: Option<u32>,
    pub respiratory_motion_compensation_technique: String,
    pub respiratory_signal_source: String,
    pub crc: i32,
}

impl Default for SiteSetup {
    fn default() -> Self {
        Self {
            rx_site_name: "".to_string(),
            patient_orientation: "".to_string(),
            treatment_machine: "".to_string(),
            tolerance_table: "".to_string(),
            isocenter_position_x: None,
            isocenter_position_y: None,
            isocenter_position_z: None,
            structure_set_uid: "".to_string(),
            frame_of_reference_uid: "".to_string(),
            couch_vertical: None,
            couch_lateral: None,
            couch_longitudinal: None,
            couch_angle: None,
            couch_pedestal: None,
            table_top_vertical_displacement: None,
            table_top_longitudinal_displacement: None,
            table_top_lateral_displacement: None,
            mrl_coil_name: "".to_string(),
            mrl_coil_index: None,
            couch_reference: "".to_string(),
            couch_reference_index: None,
            respiratory_motion_compensation_technique: "".to_string(),
            respiratory_signal_source: "".to_string(),
            crc: 0
        }
    }
}

impl SiteSetup {
   pub fn new() -> Self {
       Self::default()
   }
}

