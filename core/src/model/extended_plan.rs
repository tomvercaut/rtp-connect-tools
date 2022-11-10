#[derive(Debug, Clone)]
pub struct ExtendedPlan {
    pub encoding: String,
    pub fullname: String,
    pub patient_comments: String,
    pub crc: i32,
}

impl Default for ExtendedPlan {
    fn default() -> Self {
        Self {
            encoding: "".to_string(),
            fullname: "".to_string(),
            patient_comments: "".to_string(),
            crc: 0
        }
    }
}

impl ExtendedPlan {
    pub fn new() -> Self {
        Self::default()
    }
}