#[derive(Debug, Clone)]
pub struct MlcShape {
    pub field_id: String,
    pub control_pt_number: u32,
    pub total_shape_points: u32,
    pub coordinates_x: Vec<f64>,
    pub coordinates_y: Vec<f64>,
    pub crc: i32,
}

impl std::default::Default for MlcShape {
    fn default() -> Self {
        Self {
            field_id: "".to_string(),
            control_pt_number: 0,
            total_shape_points: 0,
            coordinates_x: vec![],
            coordinates_y: vec![],
            crc: 0
        }
    }
}

impl MlcShape {
    pub fn new() -> Self {
        Self::default()
    }
}