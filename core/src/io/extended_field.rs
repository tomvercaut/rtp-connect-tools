use crate::io::Error;
use crate::model::ExtendedField;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    FieldId,
    OriginalPlanUid,
    OriginalBeamNumber,
    OriginalBeamName,
    IsFff,
    AccessoryCode,
    AccessoryType,
    HighDoseAuthorization,
    ReferencedRtPlanUid,
    RtPlanRelationship,
    Crc,
}


/// Parse the data elements from an ExtendedField Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: extended field definition record parameters
///
/// returns: Result<ExtendedField, Error>
pub fn parse(params: &Vec<String>) -> Result<ExtendedField, Error> {
    let mut field = ExtendedField::default();
    let expected_length = 12;
    let expected_keyword = "EXTENDED_FIELD_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    field.field_id = params[Id::FieldId as usize].to_string();
    field.original_plan_uid = params[Id::OriginalPlanUid as usize].to_string();
    field.original_beam_number = params[Id::OriginalBeamNumber as usize].to_string();
    field.original_beam_name = params[Id::OriginalBeamName as usize].to_string();
    field.is_fff = params[Id::IsFff as usize].to_string();
    field.accessory_code = params[Id::AccessoryCode as usize].to_string();
    field.accessory_type = params[Id::AccessoryType as usize].to_string();
    field.high_dose_authorization = params[Id::HighDoseAuthorization as usize].to_string();
    field.referenced_rt_plan_uid = params[Id::ReferencedRtPlanUid as usize].to_string();
    field.rt_plan_relationship = params[Id::RtPlanRelationship as usize].to_string();
    field.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(field)
}


#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "EXTENDED_FIELD_DEF".to_string(),
            "FieldId".to_string(),
            "OriginalPlanUid".to_string(),
            "OriginalBeamNumber".to_string(),
            "OriginalBeamName".to_string(),
            "IsFff".to_string(),
            "AccessoryCode".to_string(),
            "AccessoryType".to_string(),
            "HighDoseAuthorization".to_string(),
            "ReferencedRtPlanUid".to_string(),
            "RtPlanRelationship".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let field = res.unwrap();
        assert_eq!(&field.field_id, "FieldId");
        assert_eq!(&field.original_plan_uid, "OriginalPlanUid");
        assert_eq!(&field.original_beam_number, "OriginalBeamNumber");
        assert_eq!(&field.original_beam_name, "OriginalBeamName");
        assert_eq!(&field.is_fff, "IsFff");
        assert_eq!(&field.accessory_code, "AccessoryCode");
        assert_eq!(&field.accessory_type, "AccessoryType");
        assert_eq!(&field.high_dose_authorization, "HighDoseAuthorization");
        assert_eq!(&field.rt_plan_relationship, "RtPlanRelationship");
        assert_eq!(field.crc, 1234);
    }
}
