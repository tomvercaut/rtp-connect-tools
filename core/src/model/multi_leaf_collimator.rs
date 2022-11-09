#[derive(Debug, Clone)]
pub struct MultiLeafCollimator {
    pub field_id: String,
    pub mlc_type: String,
    pub mlc_leaves: u32,
    pub mlc_a: Vec<f64>,
    pub mlc_b: Vec<f64>,
    pub crc: i32,
}

impl std::default::Default for MultiLeafCollimator {
    fn default() -> Self {
        Self {
            field_id: "".to_string(),
            mlc_type: "".to_string(),
            mlc_leaves: 0,
            mlc_a: vec![],
            mlc_b: vec![],
            crc: 0
        }
    }
}

impl MultiLeafCollimator {
    pub fn new() -> Self {
        Self::default()
    }
}