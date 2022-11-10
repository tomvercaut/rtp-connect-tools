use crate::io::control_point::Id::{MlcA, MlcB};
use crate::io::{Error, opt_f64, opt_u32};
use crate::model::{ControlPoint, Rotation};

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    FieldId,
    MlcType,
    MlcLeaves,
    TotalControlPoints,
    ControlPtNumber,
    MuConvention,
    MonitorUnits,
    WedgePosition,
    Energy,
    DoseRate,
    Ssd,
    ScaleConvention,
    GantryAngle,
    GantryDir,
    CollimatorAngle,
    CollimatorDir,
    FieldXMode,
    FieldX,
    CollimatorX1,
    CollimatorX2,
    FieldYMode,
    FieldY,
    CollimatorY1,
    CollimatorY2,
    CouchVertical,
    CouchLateral,
    CouchLongitudinal,
    CouchAngle,
    CouchDir,
    CouchPedestal,
    CouchPedDir,
    IsocenterPositionX,
    IsocenterPositionY,
    IsocenterPositionZ,
    MlcA,
    MlcB = MlcA as usize + 100,
    Crc = MlcB as usize + 100,
}


/// Parse the data elements from a Control Point Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: control point record parameters
///
/// returns: Result<ControlPoint, Error>
pub fn parse(params: &Vec<String>) -> Result<ControlPoint, Error> {
    let mut cp = ControlPoint::default();
    let expected_length = 236;
    let expected_keyword = "CONTROL_PT_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    cp.field_id = params[Id::FieldId as usize].to_string();
    cp.mlc_type = params[Id::MlcType as usize].to_string();
    cp.mlc_leaves = opt_u32(params[Id::MlcLeaves as usize].as_str()).unwrap();
    cp.total_control_points = opt_u32(params[Id::TotalControlPoints as usize].as_str()).unwrap();
    cp.control_pt_number = opt_u32(params[Id::ControlPtNumber as usize].as_str()).unwrap();
    cp.mu_convention = opt_u32(params[Id::MuConvention as usize].as_str());
    cp.monitor_units = opt_f64(params[Id::MonitorUnits as usize].as_str());
    cp.wedge_position = params[Id::WedgePosition as usize].to_string();
    cp.energy = opt_f64(params[Id::Energy as usize].as_str());
    cp.dose_rate = opt_f64(params[Id::DoseRate as usize].as_str());
    cp.ssd = opt_f64(params[Id::Ssd as usize].as_str());
    cp.scale_convention = opt_u32(params[Id::ScaleConvention as usize].as_str());
    cp.gantry_angle = opt_f64(params[Id::GantryAngle as usize].as_str());
    cp.gantry_dir = Rotation::try_from(params[Id::GantryDir as usize].as_str()).ok();
    cp.collimator_angle = opt_f64(params[Id::CollimatorAngle as usize].as_str());
    cp.collimator_dir = Rotation::try_from(params[Id::CollimatorDir as usize].as_str()).ok();
    cp.field_x_mode = params[Id::FieldXMode as usize].to_string();
    cp.field_x = opt_f64(params[Id::FieldX as usize].as_str());
    cp.collimator_x1 = opt_f64(params[Id::CollimatorX1 as usize].as_str());
    cp.collimator_x2 = opt_f64(params[Id::CollimatorX2 as usize].as_str());
    cp.field_y_mode = params[Id::FieldYMode as usize].to_string();
    cp.field_y = opt_f64(params[Id::FieldY as usize].as_str());
    cp.collimator_y1 = opt_f64(params[Id::CollimatorY1 as usize].as_str());
    cp.collimator_y2 = opt_f64(params[Id::CollimatorY2 as usize].as_str());
    cp.couch_vertical = opt_f64(params[Id::CouchVertical as usize].as_str());
    cp.couch_lateral = opt_f64(params[Id::CouchLateral as usize].as_str());
    cp.couch_longitudinal = opt_f64(params[Id::CouchLongitudinal as usize].as_str());
    cp.couch_angle = opt_f64(params[Id::CouchAngle as usize].as_str());
    cp.couch_dir = Rotation::try_from(params[Id::CouchDir as usize].as_str()).ok();
    cp.couch_pedestal = opt_f64(params[Id::CouchPedestal as usize].as_str());
    cp.couch_pedestal_dir = Rotation::try_from(params[Id::CouchPedDir as usize].as_str()).ok();
    cp.isocenter_position_x = opt_f64(params[Id::IsocenterPositionX as usize].as_str());
    cp.isocenter_position_y = opt_f64(params[Id::IsocenterPositionY as usize].as_str());
    cp.isocenter_position_z = opt_f64(params[Id::IsocenterPositionZ as usize].as_str());
    for i in 0..cp.mlc_leaves {
        cp.mlc_a.push(opt_f64(params[MlcA as usize + i as usize].as_str()).unwrap());
        cp.mlc_b.push(opt_f64(params[MlcB as usize + i as usize].as_str()).unwrap());
    }
    cp.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(cp)
}

#[cfg(test)]
mod tests {
    use crate::io::control_point::Id::{MlcA, MlcB};
    use crate::model::Rotation;

    #[test]
    fn parse() {
        let nlp = 30u32;
        let mlp = 100u32;
        let mut params = vec![
            "CONTROL_PT_DEF".to_string(),
            "FieldId".to_string(),
            "MlcType".to_string(),
            nlp.to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "WedgePosition".to_string(),
            "10".to_string(),
            "11".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
            "CW".to_string(),
            "16".to_string(),
            "CC".to_string(),
            "FieldXMode".to_string(),
            "19".to_string(),
            "20".to_string(),
            "21".to_string(),
            "FieldYMode".to_string(),
            "23".to_string(),
            "24".to_string(),
            "25".to_string(),
            "26".to_string(),
            "27".to_string(),
            "28".to_string(),
            "29".to_string(),
            "CW".to_string(),
            "31".to_string(),
            "CC".to_string(),
            "33".to_string(),
            "34".to_string(),
            "35".to_string(),
        ];
        for i in 0..nlp {
            params.push((MlcA as usize + i as usize).to_string());
        }
        for _i in nlp..mlp {
            params.push("".to_string());
        }
        for i in 0..nlp {
            params.push((MlcB as usize + i as usize).to_string());
        }
        for _i in nlp..mlp {
            params.push("".to_string());
        }
        params.push("1234".to_string());
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let cp = res.unwrap();
        assert_eq!(&cp.field_id, "FieldId");
        assert_eq!(&cp.mlc_type, "MlcType");
        assert_eq!(cp.mlc_leaves, nlp);
        assert_eq!(cp.total_control_points, 5);
        assert_eq!(cp.control_pt_number, 6);
        assert_eq!(cp.mu_convention, Some(7));
        assert_eq!(cp.monitor_units, Some(8.0));
        assert_eq!(&cp.wedge_position, "WedgePosition");
        assert_eq!(cp.energy, Some(10.0));
        assert_eq!(cp.dose_rate, Some(11.0));
        assert_eq!(cp.ssd, Some(12.0));
        assert_eq!(cp.ssd, Some(12.0));
        assert_eq!(cp.scale_convention, Some(13));
        assert_eq!(cp.gantry_angle, Some(14.0));
        assert_eq!(cp.gantry_dir, Some(Rotation::CW));
        assert_eq!(cp.collimator_angle, Some(16.0));
        assert_eq!(cp.collimator_dir, Some(Rotation::CC));
        assert_eq!(&cp.field_x_mode, "FieldXMode");
        assert_eq!(cp.field_x, Some(19.0));
        assert_eq!(cp.collimator_x1, Some(20.0));
        assert_eq!(cp.collimator_x2, Some(21.0));
        assert_eq!(&cp.field_y_mode, "FieldYMode");
        assert_eq!(cp.field_y, Some(23.0));
        assert_eq!(cp.collimator_y1, Some(24.0));
        assert_eq!(cp.collimator_y2, Some(25.0));
        assert_eq!(cp.couch_vertical, Some(26.0));
        assert_eq!(cp.couch_lateral, Some(27.0));
        assert_eq!(cp.couch_longitudinal, Some(28.0));
        assert_eq!(cp.couch_angle, Some(29.0));
        assert_eq!(cp.couch_dir, Some(Rotation::CW));
        assert_eq!(cp.couch_pedestal, Some(31.0));
        assert_eq!(cp.couch_pedestal_dir, Some(Rotation::CC));
        assert_eq!(cp.isocenter_position_x, Some(33.0));
        assert_eq!(cp.isocenter_position_y, Some(34.0));
        assert_eq!(cp.isocenter_position_z, Some(35.0));
        assert_eq!(cp.mlc_a.len(), nlp as usize);
        assert_eq!(cp.mlc_b.len(), nlp as usize);
        for i in 0..nlp {
            assert_eq!(cp.mlc_a[i as usize], (MlcA as usize + i as usize) as f64);
            assert_eq!(cp.mlc_b[i as usize], (MlcB as usize + i as usize) as f64);
        }
        assert_eq!(cp.crc, 1234);
    }
}