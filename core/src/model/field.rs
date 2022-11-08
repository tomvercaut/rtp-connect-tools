#[derive(Debug, Clone)]
pub struct Field {
    pub rx_site_name: String,
    pub field_name: String,
    pub field_id: String,
    pub field_note: String,
    pub field_dose: Option<f64>,
    pub field_monitor_units: Option<f64>,
    pub wedge_monitor_units: Option<f64>,
    pub treatment_machine: String,
    pub treatment_type: String,
    pub modality: String,
    pub energy: Option<f64>,
    pub time: Option<u32>,
    pub dose_rate: Option<f64>,
    pub sad: Option<f64>,
    pub ssd: Option<f64>,
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
    pub tolerance_table: String,
    pub arc_direction: String,
    pub arc_start_angle: Option<f64>,
    pub arc_stop_angle: Option<f64>,
    pub arc_mu_degree: Option<f64>,
    pub wedge: String,
    pub dynamic_wedge: String,
    pub block: String,
    pub compensator: String,
    pub e_applicator: String,
    pub e_field_def_aperture: String,
    pub bolus: String,
    pub portfilm_mu_open: Option<f64>,
    pub portfilm_coeff_open: Option<f64>,
    pub portfilm_delta_open: Option<f64>,
    pub portfilm_mu_treat: Option<f64>,
    pub portfilm_coeff_treat: Option<f64>,
    pub isocenter_position_x: Option<f64>,
    pub isocenter_position_y: Option<f64>,
    pub isocenter_position_z: Option<f64>,
    pub crc: i32,
}

impl std::default::Default for Field {
    fn default() -> Self {
        Self {
            rx_site_name: "".to_string(),
            field_name: "".to_string(),
            field_id: "".to_string(),
            field_note: "".to_string(),
            field_dose: None,
            field_monitor_units: None,
            wedge_monitor_units: None,
            treatment_machine: "".to_string(),
            treatment_type: "".to_string(),
            modality: "".to_string(),
            energy: None,
            time: None,
            dose_rate: None,
            sad: None,
            ssd: None,
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
            tolerance_table: "".to_string(),
            arc_direction: "".to_string(),
            arc_start_angle: None,
            arc_stop_angle: None,
            arc_mu_degree: None,
            wedge: "".to_string(),
            dynamic_wedge: "".to_string(),
            block: "".to_string(),
            compensator: "".to_string(),
            e_applicator: "".to_string(),
            e_field_def_aperture: "".to_string(),
            bolus: "".to_string(),
            portfilm_mu_open: None,
            portfilm_coeff_open: None,
            portfilm_delta_open: None,
            portfilm_mu_treat: None,
            portfilm_coeff_treat: None,
            isocenter_position_x: None,
            isocenter_position_y: None,
            isocenter_position_z: None,
            crc: 0,
        }
    }
}

impl Field {
    pub fn new() -> Self {
        Self::default()
    }

}