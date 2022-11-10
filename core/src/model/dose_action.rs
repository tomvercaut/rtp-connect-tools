#[derive(Debug, Clone)]
pub struct DoseAction {
    pub region_name: String,
    pub action_dose: Option<f64>,
    pub action_note: String,
    pub crc: i32,
}

impl std::default::Default for DoseAction {
    fn default() -> Self {
        Self {
            region_name: "".to_string(),
            action_dose: None,
            action_note: "".to_string(),
            crc: 0
        }
    }
}

impl DoseAction {
    pub fn new() -> Self {
        Self::default()
    }
}