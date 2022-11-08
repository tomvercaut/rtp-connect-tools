use crate::io::Error;
use crate::model::{ExtendedPlan};

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    Encoding,
    Fullname,
    PatientComments,
    Crc
}

/// Parse the data elements from an ExtendedPlan Definition Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: extended plan definition record parameters
///
/// returns: Result<ExtendedPlan, Error>
pub fn parse(params: &Vec<String>) -> Result<ExtendedPlan, Error> {
    let mut plan = ExtendedPlan::default();
    let expected_length = 5;
    let expected_keyword = "EXTENDED_PLAN_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    plan.encoding = params[Id::Encoding as usize].to_string();
    plan.fullname = params[Id::Fullname as usize].to_string();
    plan.patient_comments = params[Id::PatientComments as usize].to_string();
    plan.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(plan)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "EXTENDED_PLAN_DEF".to_string(),
            "BASE64".to_string(),
            "Fullname".to_string(),
            "PatientComments".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let plan = res.unwrap();
        assert_eq!(&plan.encoding, "BASE64");
        assert_eq!(&plan.fullname, "Fullname");
        assert_eq!(&plan.patient_comments, "PatientComments");
        assert_eq!(plan.crc, 1234);
    }
}
