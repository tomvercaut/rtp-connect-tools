use crate::io::{Error, opt_f64, opt_u32};
use crate::model::{SiteSetup};

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    RxSiteName,
    PatientOrientation,
    TreatmentMachine,
    ToleranceTable,
    IsocenterPositionX,
    IsocenterPositionY,
    IsocenterPositionZ,
    StructureSetUid,
    FrameOfReferenceUid,
    CouchVertical,
    CouchLateral,
    CouchLongitudinal,
    CouchAngle,
    CouchPedestal,
    TableTopVerticalDisplacement,
    TableTopLongitudinalDisplacement,
    TableTopLateralDisplacement,
    MrlCoilName,
    MrlCoilIndex,
    CouchReference,
    CouchReferenceIndex,
    RespiratoryMotionCompensationTechnique,
    RespiratorySignalSource,
    Crc,
}


/// Parse the data elements from an Site Setup Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: site setup record parameters
///
/// returns: Result<SiteSetup, Error>
pub fn parse(params: &Vec<String>) -> Result<SiteSetup, Error> {
    let mut ss = SiteSetup::default();
    let expected_length = 25;
    let expected_keyword = "SITE_SETUP_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    ss.rx_site_name = params[Id::RxSiteName as usize].to_string();
    ss.patient_orientation = params[Id::PatientOrientation as usize].to_string();
    ss.treatment_machine = params[Id::TreatmentMachine as usize].to_string();
    ss.tolerance_table = params[Id::ToleranceTable as usize].to_string();
    ss.isocenter_position_x = opt_f64(params[Id::IsocenterPositionX as usize].as_str());
    ss.isocenter_position_y = opt_f64(params[Id::IsocenterPositionY as usize].as_str());
    ss.isocenter_position_z = opt_f64(params[Id::IsocenterPositionZ as usize].as_str());
    ss.structure_set_uid = params[Id::StructureSetUid as usize].to_string();
    ss.frame_of_reference_uid = params[Id::FrameOfReferenceUid as usize].to_string();
    ss.couch_vertical = opt_f64(params[Id::CouchVertical as usize].as_str());
    ss.couch_lateral = opt_f64(params[Id::CouchLateral as usize].as_str());
    ss.couch_longitudinal = opt_f64(params[Id::CouchLongitudinal as usize].as_str());
    ss.couch_angle = opt_f64(params[Id::CouchAngle as usize].as_str());
    ss.couch_pedestal = opt_f64(params[Id::CouchPedestal as usize].as_str());
    ss.table_top_vertical_displacement = opt_f64(params[Id::TableTopVerticalDisplacement as usize].as_str());
    ss.table_top_longitudinal_displacement = opt_f64(params[Id::TableTopLongitudinalDisplacement as usize].as_str());
    ss.table_top_lateral_displacement = opt_f64(params[Id::TableTopLateralDisplacement as usize].as_str());
    ss.mrl_coil_name = params[Id::MrlCoilName as usize].to_string();
    ss.mrl_coil_index = opt_u32(params[Id::MrlCoilIndex as usize].as_str());
    ss.couch_reference = params[Id::CouchReference as usize].to_string();
    ss.couch_reference_index = opt_u32(params[Id::CouchReferenceIndex as usize].as_str());
    ss.respiratory_motion_compensation_technique = params[Id::RespiratoryMotionCompensationTechnique as usize].to_string();
    ss.respiratory_signal_source = params[Id::RespiratorySignalSource as usize].to_string();
    ss.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(ss)
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "SITE_SETUP_DEF".to_string(),
            "RxSiteName".to_string(),
            "HFS".to_string(),
            "TreatmentMachine".to_string(),
            "ToleranceTable".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "StructureSetUid".to_string(),
            "FrameOfReferenceUid".to_string(),
            "11".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
            "15".to_string(),
            "16".to_string(),
            "17".to_string(),
            "18".to_string(),
            "MrlCoilName".to_string(),
            "20".to_string(),
            "CouchReference".to_string(),
            "22".to_string(),
            "RespiratoryMotionCompensationTechnique".to_string(),
            "RespiratorySignalSource".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let plan = res.unwrap();
        assert_eq!(&plan.rx_site_name, "RxSiteName");
        assert_eq!(&plan.patient_orientation, "HFS");
        assert_eq!(&plan.treatment_machine, "TreatmentMachine");
        assert_eq!(&plan.tolerance_table, "ToleranceTable");
        assert_eq!(plan.isocenter_position_x, Some(6.0));
        assert_eq!(plan.isocenter_position_y, Some(7.0));
        assert_eq!(plan.isocenter_position_z, Some(8.0));
        assert_eq!(&plan.structure_set_uid, "StructureSetUid");
        assert_eq!(&plan.frame_of_reference_uid, "FrameOfReferenceUid");
        assert_eq!(plan.couch_vertical, Some(11.0));
        assert_eq!(plan.couch_lateral, Some(12.0));
        assert_eq!(plan.couch_longitudinal, Some(13.0));
        assert_eq!(plan.couch_angle, Some(14.0));
        assert_eq!(plan.couch_pedestal, Some(15.0));
        assert_eq!(plan.table_top_vertical_displacement, Some(16.0));
        assert_eq!(plan.table_top_longitudinal_displacement, Some(17.0));
        assert_eq!(plan.table_top_lateral_displacement, Some(18.0));
        assert_eq!(&plan.mrl_coil_name, "MrlCoilName");
        assert_eq!(plan.mrl_coil_index, Some(20));
        assert_eq!(&plan.couch_reference, "CouchReference");
        assert_eq!(plan.couch_reference_index, Some(22));
        assert_eq!(&plan.respiratory_motion_compensation_technique, "RespiratoryMotionCompensationTechnique");
        assert_eq!(&plan.respiratory_signal_source, "RespiratorySignalSource");
    }
}