#[derive(Debug, Clone)]
pub struct Prescription {
    pub course_id: String,
    pub rx_site_name: String,
    pub technique: String,
    pub modality: String,
    pub dose_spec: String,
    pub rx_depth: Option<f64>,
    pub dose_ttl: Option<f64>,
    pub dose_tx: Option<f64>,
    pub pattern: String,
    pub rx_note: String,
    pub number_of_fields: u32,
    pub crc: i32,
}

impl Default for Prescription {
    fn default() -> Self {
        Self {
            course_id: "".to_string(),
            rx_site_name: "".to_string(),
            technique: "".to_string(),
            modality: "".to_string(),
            dose_spec: "".to_string(),
            rx_depth: None,
            dose_ttl: None,
            dose_tx: None,
            pattern: "".to_string(),
            rx_note: "".to_string(),
            number_of_fields: 0,
            crc: 0
        }
    }
}

impl Prescription {
   pub fn new() -> Self {
       Self::default()
   }
}