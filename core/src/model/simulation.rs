#[derive(Debug, Clone)]
pub struct Simulation {
    pub rx_site_name: String,
    pub field_name: String,
    pub field_id: String,
    pub field_note: String,
    pub treatment_machine: String,
    pub gantry_angle: Option<f64>,
    pub collimator_angle: Option<f64>,
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
    pub couch_pedestal: Option<f64>,
    pub sad: Option<f64>,
    pub ap_separation: Option<f64>,
    pub pa_separation: Option<f64>,
    pub lateral_separation: Option<f64>,
    pub tangential_separation: Option<f64>,
    pub other_label_1: String,
    pub ssd_1: Option<f64>,
    pub sfd_1: Option<f64>,
    pub other_label_2: String,
    pub other_measurement_1: String,
    pub other_measurement_2: String,
    pub other_label_3: String,
    pub other_measurement_3: String,
    pub other_measurement_4: String,
    pub other_label_4: String,
    pub other_measurement_5: String,
    pub other_measurement_6: String,
    pub blade_x_mode: String,
    pub blade_x: Option<f64>,
    pub blade_x1: Option<f64>,
    pub blade_x2: Option<f64>,
    pub blade_y_mode: String,
    pub blade_y: Option<f64>,
    pub blade_y1: Option<f64>,
    pub blade_y2: Option<f64>,
    pub ii_lateral: Option<f64>,
    pub ii_longitudinal: Option<f64>,
    pub ii_vertical: Option<f64>,
    pub kvp: Option<f64>,
    pub ma: Option<f64>,
    pub seconds: Option<f64>,
    pub crc: i32,
}

impl std::default::Default for Simulation {
    fn default() -> Self {
        Self {
            rx_site_name: "".to_string(),
            field_name: "".to_string(),
            field_id: "".to_string(),
            field_note: "".to_string(),
            treatment_machine: "".to_string(),
            gantry_angle: None,
            collimator_angle: None,
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
            couch_pedestal: None,
            sad: None,
            ap_separation: None,
            pa_separation: None,
            lateral_separation: None,
            tangential_separation: None,
            other_label_1: "".to_string(),
            ssd_1: None,
            sfd_1: None,
            other_label_2: "".to_string(),
            other_measurement_1: "".to_string(),
            other_measurement_2: "".to_string(),
            other_label_3: "".to_string(),
            other_measurement_3: "".to_string(),
            other_measurement_4: "".to_string(),
            other_label_4: "".to_string(),
            other_measurement_5: "".to_string(),
            other_measurement_6: "".to_string(),
            blade_x_mode: "".to_string(),
            blade_x: None,
            blade_x1: None,
            blade_x2: None,
            blade_y_mode: "".to_string(),
            blade_y: None,
            blade_y1: None,
            blade_y2: None,
            ii_lateral: None,
            ii_longitudinal: None,
            ii_vertical: None,
            kvp: None,
            ma: None,
            seconds: None,
            crc: 0
        }
    }
}

impl Simulation {
   pub fn new() -> Self {
       Self::default()
   }
}