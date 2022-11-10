use crate::model::Rotation;

#[derive(Debug, Clone)]
pub struct ControlPoint {
    pub field_id: String,
    pub mlc_type: String,
    pub mlc_leaves: u32,
    pub total_control_points: u32,
    pub control_pt_number: u32,
    pub mu_convention: Option<u32>,
    pub monitor_units: Option<f64>,
    pub wedge_position: String,
    pub energy: Option<f64>,
    pub dose_rate: Option<f64>,
    pub ssd: Option<f64>,
    pub scale_convention: Option<u32>,
    pub gantry_angle: Option<f64>,
    pub gantry_dir: Option<Rotation>,
    pub collimator_angle: Option<f64>,
    pub collimator_dir: Option<Rotation>,
    pub field_x_mode: String,
    pub field_x: Option<f64>,
    pub collimator_x1: Option<f64>,
    pub collimator_x2: Option<f64>,
    pub field_y_mode: String,
    pub field_y: Option<f64>,
    pub collimator_y1: Option<f64>,
    pub collimator_y2: Option<f64>,
    pub couch_vertical: Option<f64>,
    pub couch_lateral: Option<f64>,
    pub couch_longitudinal: Option<f64>,
    pub couch_angle: Option<f64>,
    pub couch_dir: Option<Rotation>,
    pub couch_pedestal: Option<f64>,
    pub couch_pedestal_dir: Option<Rotation>,
    pub isocenter_position_x: Option<f64>,
    pub isocenter_position_y: Option<f64>,
    pub isocenter_position_z: Option<f64>,
    pub mlc_a: Vec<f64>,
    pub mlc_b: Vec<f64>,
    pub crc: i32,
}

impl std::default::Default for ControlPoint {
    fn default() -> Self {
        Self {
            field_id: "".to_string(),
            mlc_type: "".to_string(),
            mlc_leaves: 0,
            total_control_points: 0,
            control_pt_number: 0,
            mu_convention: None,
            monitor_units: None,
            wedge_position: "".to_string(),
            energy: None,
            dose_rate: None,
            ssd: None,
            scale_convention: None,
            gantry_angle: None,
            gantry_dir: None,
            collimator_angle: None,
            collimator_dir: None,
            field_x_mode: "".to_string(),
            field_x: None,
            collimator_x1: None,
            collimator_x2: None,
            field_y_mode: "".to_string(),
            field_y: None,
            collimator_y1: None,
            collimator_y2: None,
            couch_vertical: None,
            couch_lateral: None,
            couch_longitudinal: None,
            couch_angle: None,
            couch_dir: None,
            couch_pedestal: None,
            couch_pedestal_dir: None,
            isocenter_position_x: None,
            isocenter_position_y: None,
            isocenter_position_z: None,
            mlc_a: vec![],
            mlc_b: vec![],
            crc: 0,
        }
    }
}

impl ControlPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
