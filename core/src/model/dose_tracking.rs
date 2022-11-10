#[derive(Debug, Clone)]
pub struct DoseTracking {
    pub region_name: String,
    pub region_prior_dose: Option<f64>,
    pub field_ids: Vec<String>,
    pub reg_coeffs: Vec<Option<f64>>,
    pub actual_dose: Option<f64>,
    pub actual_fractions: u32,
    pub crc: i32,
}

impl std::default::Default for DoseTracking {
    fn default() -> Self {
        Self {
            region_name: "".to_string(),
            region_prior_dose: None,
            field_ids: vec![],
            reg_coeffs: vec![],
            actual_dose: None,
            actual_fractions: 0,
            crc: 0
        }
    }
}

impl DoseTracking {
    pub fn new() -> Self {
        Self::default()
    }
}