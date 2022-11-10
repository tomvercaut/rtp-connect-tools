#[derive(Debug, Clone)]
pub struct ExtendedField {
    pub field_id: String,
    pub original_plan_uid: String,
    pub original_beam_number: String,
    pub original_beam_name: String,
    pub is_fff: String,
    pub accessory_code: String,
    pub accessory_type: String,
    pub high_dose_authorization: String,
    pub referenced_rt_plan_uid: String,
    pub rt_plan_relationship: String,
    pub crc: i32,
}

impl Default for ExtendedField {
    fn default() -> Self {
        Self {
            field_id: "".to_string(),
            original_plan_uid: "".to_string(),
            original_beam_number: "".to_string(),
            original_beam_name: "".to_string(),
            is_fff: "".to_string(),
            accessory_code: "".to_string(),
            accessory_type: "".to_string(),
            high_dose_authorization: "".to_string(),
            referenced_rt_plan_uid: "".to_string(),
            rt_plan_relationship: "".to_string(),
            crc: 0
        }
    }
}

impl ExtendedField {
    pub fn new() -> Self {
        Self::default()
    }
}