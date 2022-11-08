use crate::io::{Error, opt_f64};
use crate::model::{Prescription};

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    CourseId,
    RxSiteName,
    Technique,
    Modality,
    DoseSpec,
    RxDepth,
    DoseTtl,
    DoseTx,
    Pattern,
    RxNote,
    NumberOfFields,
    Crc,
}

/// Parse the data elements from an Prescription Site Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: prescription site record parameters
///
/// returns: Result<Prescription, Error>
pub fn parse(params: &Vec<String>) -> Result<Prescription, Error> {
    let mut presc = Prescription::default();
    let expected_length = 13;
    let expected_keyword = "RX_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    presc.course_id = params[Id::CourseId as usize].to_string();
    presc.rx_site_name = params[Id::RxSiteName as usize].to_string();
    presc.technique = params[Id::Technique as usize].to_string();
    presc.modality = params[Id::Modality as usize].to_string();
    presc.dose_spec = params[Id::DoseSpec as usize].to_string();
    presc.rx_depth = opt_f64(params[Id::RxDepth as usize].as_str());
    presc.dose_ttl = opt_f64(params[Id::DoseTtl as usize].as_str());
    presc.dose_tx = opt_f64(params[Id::DoseTx as usize].as_str());
    presc.pattern = params[Id::Pattern as usize].to_string();
    presc.rx_note = params[Id::RxNote as usize].to_string();
    presc.number_of_fields = (&params[Id::NumberOfFields as usize]).parse::<u32>()?;
    presc.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(presc)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let params = vec![
            "RX_DEF".to_string(),
            "CourseId".to_string(),
            "RxSiteName".to_string(),
            "Technique".to_string(),
            "Modality".to_string(),
            "DoseSpec".to_string(),
            "10".to_string(),
            "1000".to_string(),
            "100".to_string(),
            "Daily".to_string(),
            "RxNote".to_string(),
            "2".to_string(),
            "1234".to_string(),
        ];
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let presc = res.unwrap();
        assert_eq!(&presc.course_id, "CourseId");
        assert_eq!(&presc.rx_site_name, "RxSiteName");
        assert_eq!(&presc.technique, "Technique");
        assert_eq!(&presc.modality, "Modality");
        assert_eq!(&presc.dose_spec, "DoseSpec");
        assert_eq!(presc.rx_depth, Some(10.0));
        assert_eq!(presc.dose_ttl, Some(1000.0));
        assert_eq!(presc.dose_tx, Some(100.0));
        assert_eq!(&presc.pattern, "Daily");
        assert_eq!(&presc.rx_note, "RxNote");
        assert_eq!(presc.number_of_fields, 2);
        assert_eq!(presc.crc, 1234);
    }
}
