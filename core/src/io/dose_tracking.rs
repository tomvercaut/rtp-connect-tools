use crate::io::{Error, opt_f64, opt_u32};
use crate::io::dose_tracking::Id::{FieldId1, RegCoeff1};
use crate::model::DoseTracking;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    RegionName,
    RegionPriorDose,
    FieldId1,
    RegCoeff1,
    ActualDose = FieldId1 as usize + 20,
    ActualFractions,
    Crc,
}

/// Parse the data elements from a Dose Tracking Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: dose tracking record parameters
///
/// returns: Result<DoseTracking, Error>
pub fn parse(params: &Vec<String>) -> Result<DoseTracking, Error> {
    let mut dt = DoseTracking::default();
    let expected_length = 26;
    let expected_keyword = "DOSE_DEF";
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    dt.region_name = params[Id::RegionName as usize].to_string();
    dt.region_prior_dose = opt_f64(params[Id::RegionPriorDose as usize].as_str());
    for i in 0..10usize {
        let fid = FieldId1 as usize + i * 2;
        let cid = RegCoeff1 as usize + i * 2;
        dt.field_ids.push(params[fid].to_string());
        dt.reg_coeffs.push(opt_f64(params[cid].as_str()));
    }
    dt.actual_dose = opt_f64(params[Id::ActualDose as usize].as_str());
    dt.actual_fractions = opt_u32(params[Id::ActualFractions as usize].as_str()).unwrap_or(0);
    dt.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(dt)
}
