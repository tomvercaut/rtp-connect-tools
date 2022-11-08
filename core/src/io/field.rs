use crate::io::{Error, opt_f64, opt_u32};
use crate::model::Field;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    RxSiteName,
    FieldName,
    FieldId,
    FieldNote,
    FieldDose,
    FieldMonitorUnits,
    WedgeMonitorUnits,
    TreatmentMachine,
    TreatmentType,
    Modality,
    Energy,
    Time,
    DoseRate,
    Sad,
    Ssd,
    GantryAngle,
    CollimatorAngle,
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
    CouchPedestal,
    ToleranceTable,
    ArcDirection,
    ArcStartAngle,
    ArcStopAngle,
    ArcMuDegree,
    Wedge,
    DynamicWedge,
    Block,
    Compensator,
    EApplicator,
    EFieldDefAperture,
    Bolus,
    PortfilmMuOpen,
    PortfilmCoeffOpen,
    PortfilmDeltaOpen,
    PortfilmMuTreat,
    PortfilmCoeffTreat,
    IsocenterPositionX,
    IsocenterPositionY,
    IsocenterPositionZ,
    Crc,
}


/// Parse the data elements from a Field from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: field record parameters
///
/// returns: Result<Field, Error>
pub fn parse(params: &Vec<String>) -> Result<Field, Error> {
    let mut field = Field::default();
    let expected_length = 52;
    let expected_keyword = "FIELD_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    field.rx_site_name = params[Id::RxSiteName as usize].to_string();
    field.field_name = params[Id::FieldName as usize].to_string();
    field.field_id = params[Id::FieldId as usize].to_string();
    field.field_note = params[Id::FieldNote as usize].to_string();
    field.field_dose = opt_f64(params[Id::FieldDose as usize].as_str());
    field.field_monitor_units = opt_f64(params[Id::FieldMonitorUnits as usize].as_str());
    field.wedge_monitor_units = opt_f64(params[Id::WedgeMonitorUnits as usize].as_str());
    field.treatment_machine = params[Id::TreatmentMachine as usize].to_string();
    field.treatment_type = params[Id::TreatmentType as usize].to_string();
    field.modality = params[Id::Modality as usize].to_string();
    field.energy = opt_f64(params[Id::Energy as usize].as_str());
    field.time = opt_u32(params[Id::Time as usize].as_str());
    field.dose_rate = opt_f64(params[Id::DoseRate as usize].as_str());
    field.sad = opt_f64(params[Id::Sad as usize].as_str());
    field.ssd = opt_f64(params[Id::Ssd as usize].as_str());
    field.gantry_angle = opt_f64(params[Id::GantryAngle as usize].as_str());
    field.collimator_angle = opt_f64(params[Id::CollimatorAngle as usize].as_str());
    field.field_x_mode = params[Id::FieldXMode as usize].to_string();
    field.field_x = opt_f64(params[Id::FieldX as usize].as_str());
    field.collimator_x1 = opt_f64(params[Id::CollimatorX1 as usize].as_str());
    field.collimator_x2 = opt_f64(params[Id::CollimatorX2 as usize].as_str());
    field.field_y_mode = params[Id::FieldYMode as usize].to_string();
    field.field_y = opt_f64(params[Id::FieldY as usize].as_str());
    field.collimator_y1 = opt_f64(params[Id::CollimatorY1 as usize].as_str());
    field.collimator_y2 = opt_f64(params[Id::CollimatorY2 as usize].as_str());
    field.couch_vertical = opt_f64(params[Id::CouchVertical as usize].as_str());
    field.couch_lateral = opt_f64(params[Id::CouchLateral as usize].as_str());
    field.couch_longitudinal = opt_f64(params[Id::CouchLongitudinal as usize].as_str());
    field.couch_angle = opt_f64(params[Id::CouchAngle as usize].as_str());
    field.couch_pedestal = opt_f64(params[Id::CouchPedestal as usize].as_str());
    field.tolerance_table = params[Id::ToleranceTable as usize].to_string();
    field.arc_direction = params[Id::ArcDirection as usize].to_string();
    field.arc_start_angle = opt_f64(params[Id::ArcStartAngle as usize].as_str());
    field.arc_stop_angle = opt_f64(params[Id::ArcStopAngle as usize].as_str());
    field.arc_mu_degree = opt_f64(params[Id::ArcMuDegree as usize].as_str());
    field.wedge = params[Id::Wedge as usize].to_string();
    field.dynamic_wedge = params[Id::DynamicWedge as usize].to_string();
    field.block = params[Id::Block as usize].to_string();
    field.compensator = params[Id::Compensator as usize].to_string();
    field.e_applicator = params[Id::EApplicator as usize].to_string();
    field.e_field_def_aperture = params[Id::EFieldDefAperture as usize].to_string();
    field.bolus = params[Id::Bolus as usize].to_string();
    field.portfilm_mu_open = opt_f64(params[Id::PortfilmMuOpen as usize].as_str());
    field.portfilm_coeff_open = opt_f64(params[Id::PortfilmCoeffOpen as usize].as_str());
    field.portfilm_delta_open = opt_f64(params[Id::PortfilmDeltaOpen as usize].as_str());
    field.portfilm_mu_treat = opt_f64(params[Id::PortfilmMuTreat as usize].as_str());
    field.portfilm_coeff_treat = opt_f64(params[Id::PortfilmCoeffTreat as usize].as_str());
    field.isocenter_position_x = opt_f64(params[Id::IsocenterPositionX as usize].as_str());
    field.isocenter_position_y = opt_f64(params[Id::IsocenterPositionY as usize].as_str());
    field.isocenter_position_z = opt_f64(params[Id::IsocenterPositionZ as usize].as_str());
    field.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(field)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "FIELD_DEF".to_string(),
            "RxSiteName".to_string(),
            "FieldName".to_string(),
            "FieldId".to_string(),
            "FieldNote".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "TreatmentMachine".to_string(),
            "TreatmentType".to_string(),
            "Modality".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
            "15".to_string(),
            "16".to_string(),
            "17".to_string(),
            "18".to_string(),
            "FieldXMode".to_string(),
            "20".to_string(),
            "21".to_string(),
            "22".to_string(),
            "FieldYMode".to_string(),
            "24".to_string(),
            "25".to_string(),
            "26".to_string(),
            "27".to_string(),
            "28".to_string(),
            "29".to_string(),
            "30".to_string(),
            "31".to_string(),
            "ToleranceTable".to_string(),
            "ArcDirection".to_string(),
            "34".to_string(),
            "35".to_string(),
            "36".to_string(),
            "Wedge".to_string(),
            "DynamicWedge".to_string(),
            "Block".to_string(),
            "Compensator".to_string(),
            "eApplicator".to_string(),
            "eFieldDefAperture".to_string(),
            "Bolus".to_string(),
            "44".to_string(),
            "45".to_string(),
            "46".to_string(),
            "47".to_string(),
            "48".to_string(),
            "49".to_string(),
            "50".to_string(),
            "51".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let field = res.unwrap();

        assert_eq!(field.rx_site_name , "RxSiteName".to_string());
        assert_eq!(field.field_name , "FieldName".to_string());
        assert_eq!(field.field_id , "FieldId".to_string());
        assert_eq!(field.field_note , "FieldNote".to_string());
        assert_eq!(field.field_dose , Some(6.0));
        assert_eq!(field.field_monitor_units , Some(7.0));
        assert_eq!(field.wedge_monitor_units , Some(8.0));
        assert_eq!(field.treatment_machine , "TreatmentMachine".to_string());
        assert_eq!(field.treatment_type , "TreatmentType".to_string());
        assert_eq!(field.modality , "Modality".to_string());
        assert_eq!(field.energy , Some(12.0));
        assert_eq!(field.time , Some(13u32));
        assert_eq!(field.dose_rate , Some(14.0));
        assert_eq!(field.sad , Some(15.0));
        assert_eq!(field.ssd , Some(16.0));
        assert_eq!(field.gantry_angle , Some(17.0));
        assert_eq!(field.collimator_angle , Some(18.0));
        assert_eq!(field.field_x_mode , "FieldXMode".to_string());
        assert_eq!(field.field_x , Some(20.0));
        assert_eq!(field.collimator_x1 , Some(21.0));
        assert_eq!(field.collimator_x2 , Some(22.0));
        assert_eq!(field.field_y_mode , "FieldYMode");
        assert_eq!(field.field_y , Some(24.0));
        assert_eq!(field.collimator_y1 , Some(25.0));
        assert_eq!(field.collimator_y2 , Some(26.0));
        assert_eq!(field.couch_vertical , Some(27.0));
        assert_eq!(field.couch_lateral , Some(28.0));
        assert_eq!(field.couch_longitudinal , Some(29.0));
        assert_eq!(field.couch_angle , Some(30.0));
        assert_eq!(field.couch_pedestal , Some(31.0));
        assert_eq!(field.tolerance_table , "ToleranceTable");
        assert_eq!(field.arc_direction , "ArcDirection");
        assert_eq!(field.arc_start_angle , Some(34.0));
        assert_eq!(field.arc_stop_angle , Some(35.0));
        assert_eq!(field.arc_mu_degree , Some(36.0));
        assert_eq!(field.wedge , "Wedge".to_string());
        assert_eq!(field.dynamic_wedge , "DynamicWedge".to_string());
        assert_eq!(field.block , "Block".to_string());
        assert_eq!(field.compensator , "Compensator".to_string());
        assert_eq!(field.e_applicator , "eApplicator".to_string());
        assert_eq!(field.e_field_def_aperture , "eFieldDefAperture".to_string());
        assert_eq!(field.bolus , "Bolus".to_string());
        assert_eq!(field.portfilm_mu_open , Some(44.0));
        assert_eq!(field.portfilm_coeff_open , Some(45.0));
        assert_eq!(field.portfilm_delta_open , Some(46.0));
        assert_eq!(field.portfilm_mu_treat , Some(47.0));
        assert_eq!(field.portfilm_coeff_treat , Some(48.0));
        assert_eq!(field.isocenter_position_x , Some(49.0));
        assert_eq!(field.isocenter_position_y , Some(50.0));
        assert_eq!(field.isocenter_position_z , Some(51.0));
        assert_eq!(field.crc , 1234);

    }
}