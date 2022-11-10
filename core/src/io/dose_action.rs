use crate::io::{Error, opt_f64};
use crate::model::DoseAction;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    RegionName,
    ActionDose,
    ActionNote,
    Crc,
}

/// Parse the data elements from a Dose Action Point from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: dose action point parameters
///
/// returns: Result<DoseAction, Error>
pub fn parse(params: &Vec<String>) -> Result<DoseAction, Error> {
    let mut da = DoseAction::default();
    let expected_length = 5;
    let expected_keyword = "DOSE_ACTION";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    da.region_name = params[Id::RegionName as usize].to_string();
    da.action_dose = opt_f64(params[Id::ActionDose as usize].as_str());
    da.action_note = params[Id::ActionNote as usize].to_string();
    da.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(da)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "DOSE_ACTION".to_string(),
            "RegionName".to_string(),
            "3".to_string(),
            "ActionNote".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let da = res.unwrap();
        assert_eq!(&da.region_name, "RegionName");
        assert_eq!(da.action_dose, Some(3.0));
        assert_eq!(&da.action_note, "ActionNote");
        assert_eq!(da.crc, 1234);
    }
}