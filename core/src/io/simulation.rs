use crate::io::{Error, opt_f64};
use crate::model::Simulation;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    RxSiteName,
    FieldName,
    FieldId,
    FieldNote,
    TreatmentMachine,
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
    Sad,
    ApSeparation,
    PaSeparation,
    LateralSeparation,
    TangentialSeparation,
    OtherLabel1,
    Ssd1,
    Sfd1,
    OtherLabel2,
    OtherMeasurement1,
    OtherMeasurement2,
    OtherLabel3,
    OtherMeasurement3,
    OtherMeasurement4,
    OtherLabel4,
    OtherMeasurement5,
    OtherMeasurement6,
    BladeXMode,
    BladeX,
    BladeX1,
    BladeX2,
    BladeYMode,
    BladeY,
    BladeY1,
    BladeY2,
    IiLateral,
    IiLongitudinal,
    IiVertical,
    Kvp,
    Ma,
    Seconds,
    Crc,
}


/// Parse the data elements from an ExtendedPlan Definition Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: extended plan definition record parameters
///
/// returns: Result<ExtendedPlan, Error>
pub fn parse(params: &Vec<String>) -> Result<Simulation, Error> {
    let mut sim = Simulation::default();
    let expected_length = 53;
    let expected_keyword = "SIM_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    sim.rx_site_name = params[Id::RxSiteName as usize].to_string();
    sim.field_name = params[Id::FieldName as usize].to_string();
    sim.field_id = params[Id::FieldId as usize].to_string();
    sim.field_note = params[Id::FieldNote as usize].to_string();
    sim.treatment_machine = params[Id::TreatmentMachine as usize].to_string();
    sim.gantry_angle = opt_f64(params[Id::GantryAngle as usize].as_str());
    sim.collimator_angle = opt_f64(params[Id::CollimatorAngle as usize].as_str());
    sim.field_x_mode = params[Id::FieldXMode as usize].to_string();
    sim.field_x = opt_f64(params[Id::FieldX as usize].as_str());
    sim.collimator_x1 = opt_f64(params[Id::CollimatorX1 as usize].as_str());
    sim.collimator_x2 = opt_f64(params[Id::CollimatorX2 as usize].as_str());
    sim.field_y_mode = params[Id::FieldYMode as usize].to_string();
    sim.field_y = opt_f64(params[Id::FieldY as usize].as_str());
    sim.collimator_y1 = opt_f64(params[Id::CollimatorY1 as usize].as_str());
    sim.collimator_y2 = opt_f64(params[Id::CollimatorY2 as usize].as_str());
    sim.couch_vertical = opt_f64(params[Id::CouchVertical as usize].as_str());
    sim.couch_lateral = opt_f64(params[Id::CouchLateral as usize].as_str());
    sim.couch_longitudinal = opt_f64(params[Id::CouchLongitudinal as usize].as_str());
    sim.couch_angle = opt_f64(params[Id::CouchAngle as usize].as_str());
    sim.couch_pedestal = opt_f64(params[Id::CouchPedestal as usize].as_str());
    sim.sad = opt_f64(params[Id::Sad as usize].as_str());
    sim.ap_separation = opt_f64(params[Id::ApSeparation as usize].as_str());
    sim.pa_separation = opt_f64(params[Id::PaSeparation as usize].as_str());
    sim.lateral_separation = opt_f64(params[Id::LateralSeparation as usize].as_str());
    sim.tangential_separation = opt_f64(params[Id::TangentialSeparation as usize].as_str());
    sim.other_label_1 = params[Id::OtherLabel1 as usize].to_string();
    sim.ssd_1 = opt_f64(params[Id::Ssd1 as usize].as_str());
    sim.sfd_1 = opt_f64(params[Id::Sfd1 as usize].as_str());
    sim.other_label_2 = params[Id::OtherLabel2 as usize].to_string();
    sim.other_measurement_1 = params[Id::OtherMeasurement1 as usize].to_string();
    sim.other_measurement_2 = params[Id::OtherMeasurement2 as usize].to_string();
    sim.other_label_3 = params[Id::OtherLabel3 as usize].to_string();
    sim.other_measurement_3 = params[Id::OtherMeasurement3 as usize].to_string();
    sim.other_measurement_4 = params[Id::OtherMeasurement4 as usize].to_string();
    sim.other_label_4 = params[Id::OtherLabel4 as usize].to_string();
    sim.other_measurement_5 = params[Id::OtherMeasurement5 as usize].to_string();
    sim.other_measurement_6 = params[Id::OtherMeasurement6 as usize].to_string();
    sim.blade_x_mode = params[Id::BladeXMode as usize].to_string();
    sim.blade_x = opt_f64(params[Id::BladeX as usize].as_str());
    sim.blade_x1 = opt_f64(params[Id::BladeX1 as usize].as_str());
    sim.blade_x2 = opt_f64(params[Id::BladeX2 as usize].as_str());
    sim.blade_y_mode = params[Id::BladeYMode as usize].to_string();
    sim.blade_y = opt_f64(params[Id::BladeY as usize].as_str());
    sim.blade_y1 = opt_f64(params[Id::BladeY1 as usize].as_str());
    sim.blade_y2 = opt_f64(params[Id::BladeY2 as usize].as_str());
    sim.ii_lateral = opt_f64(params[Id::IiLateral as usize].as_str());
    sim.ii_longitudinal = opt_f64(params[Id::IiLongitudinal as usize].as_str());
    sim.ii_vertical = opt_f64(params[Id::IiVertical as usize].as_str());
    sim.kvp = opt_f64(params[Id::Kvp as usize].as_str());
    sim.ma = opt_f64(params[Id::Ma as usize].as_str());
    sim.seconds = opt_f64(params[Id::Seconds as usize].as_str());
    sim.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(sim)
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "SIM_DEF".to_string(),
            "RxSiteName".to_string(),
            "FieldName".to_string(),
            "FieldId".to_string(),
            "FieldNote".to_string(),
            "TreatmentMachine".to_string(),
            "7".to_string(),
            "8".to_string(),
            "FieldXMode".to_string(),
            "10".to_string(),
            "11".to_string(),
            "12".to_string(),
            "FieldYMode".to_string(),
            "14".to_string(),
            "15".to_string(),
            "16".to_string(),
            "17".to_string(),
            "18".to_string(),
            "19".to_string(),
            "20".to_string(),
            "21".to_string(),
            "22".to_string(),
            "23".to_string(),
            "24".to_string(),
            "25".to_string(),
            "26".to_string(),
            "OtherLabel1".to_string(),
            "28".to_string(),
            "29".to_string(),
            "OtherLabel2".to_string(),
            "OtherMeasurement1".to_string(),
            "OtherMeasurement2".to_string(),
            "OtherLabel3".to_string(),
            "OtherMeasurement3".to_string(),
            "OtherMeasurement4".to_string(),
            "OtherLabel4".to_string(),
            "OtherMeasurement5".to_string(),
            "OtherMeasurement6".to_string(),
            "BladeXMode".to_string(),
            "40".to_string(),
            "41".to_string(),
            "42".to_string(),
            "BladeYMode".to_string(),
            "44".to_string(),
            "45".to_string(),
            "46".to_string(),
            "47".to_string(),
            "48".to_string(),
            "49".to_string(),
            "50".to_string(),
            "51".to_string(),
            "52".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let sim = res.unwrap();

        assert_eq!(&sim.rx_site_name , "RxSiteName");
        assert_eq!(&sim.field_name , "FieldName");
        assert_eq!(&sim.field_id , "FieldId");
        assert_eq!(&sim.field_note , "FieldNote");
        assert_eq!(&sim.treatment_machine , "TreatmentMachine");
        assert_eq!(sim.gantry_angle , Some(7.0));
        assert_eq!(sim.collimator_angle , Some(8.0));
        assert_eq!(&sim.field_x_mode , "FieldXMode");
        assert_eq!(sim.field_x , Some(10.0));
        assert_eq!(sim.collimator_x1 , Some(11.0));
        assert_eq!(sim.collimator_x2 , Some(12.0));
        assert_eq!(&sim.field_y_mode , "FieldYMode");
        assert_eq!(sim.field_y , Some(14.0));
        assert_eq!(sim.collimator_y1 , Some(15.0));
        assert_eq!(sim.collimator_y2 , Some(16.0));
        assert_eq!(sim.couch_vertical , Some(17.0));
        assert_eq!(sim.couch_lateral , Some(18.0));
        assert_eq!(sim.couch_longitudinal , Some(19.0));
        assert_eq!(sim.couch_angle , Some(20.0));
        assert_eq!(sim.couch_pedestal , Some(21.0));
        assert_eq!(sim.sad , Some(22.0));
        assert_eq!(sim.ap_separation , Some(23.0));
        assert_eq!(sim.pa_separation , Some(24.0));
        assert_eq!(sim.lateral_separation , Some(25.0));
        assert_eq!(sim.tangential_separation , Some(26.0));
        assert_eq!(&sim.other_label_1 , "OtherLabel1");
        assert_eq!(sim.ssd_1 , Some(28.0));
        assert_eq!(sim.sfd_1 , Some(29.0));
        assert_eq!(&sim.other_label_2 , "OtherLabel2");
        assert_eq!(&sim.other_measurement_1 , "OtherMeasurement1");
        assert_eq!(&sim.other_measurement_2 , "OtherMeasurement2");
        assert_eq!(&sim.other_label_3 , "OtherLabel3");
        assert_eq!(&sim.other_measurement_3 , "OtherMeasurement3");
        assert_eq!(&sim.other_measurement_4 , "OtherMeasurement4");
        assert_eq!(&sim.other_label_4 , "OtherLabel4");
        assert_eq!(&sim.other_measurement_5 , "OtherMeasurement5");
        assert_eq!(&sim.other_measurement_6 , "OtherMeasurement6");
        assert_eq!(&sim.blade_x_mode , "BladeXMode");
        assert_eq!(sim.blade_x , Some(40.0));
        assert_eq!(sim.blade_x1 , Some(41.0));
        assert_eq!(sim.blade_x2 , Some(42.0));
        assert_eq!(&sim.blade_y_mode , "BladeYMode");
        assert_eq!(sim.blade_y , Some(44.0));
        assert_eq!(sim.blade_y1 , Some(45.0));
        assert_eq!(sim.blade_y2 , Some(46.0));
        assert_eq!(sim.ii_lateral , Some(47.0));
        assert_eq!(sim.ii_longitudinal , Some(48.0));
        assert_eq!(sim.ii_vertical , Some(49.0));
        assert_eq!(sim.kvp , Some(50.0));
        assert_eq!(sim.ma , Some(51.0));
        assert_eq!(sim.seconds , Some(52.0));
        assert_eq!(sim.crc , 1234);
    }
}