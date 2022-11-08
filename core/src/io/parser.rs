use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use crate::io::{extended_field, extended_plan, field, plan, prescription, simulation, site_setup};
use crate::model::Rtp;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Keyword mismatch [expected: {0}, actual: {1}]")]
    Keyword(String, String),
    #[error("Number of parameters is incorrect [expected: {0}, actual: {1}]")]
    ParameterLengthMismatch(usize, usize),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

pub fn parse_file(filename: &str) -> Result<Rtp, Error> {
    let mut rtp = Rtp::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let params = get_params(line.as_str());
        if params.is_empty() {
            continue;
        }
        let keyword = params[0].as_str();
        match keyword {
            "PLAN_DEF" => {
                rtp.plan = plan::parse(&params)?;
            },
            "EXTENDED_PLAN_DEF" => {
                rtp.extended_plan = extended_plan::parse(&params)?;
            },
            "RX_DEF" => {
                rtp.prescription = prescription::parse(&params)?;
            },
            "SITE_SETUP_DEF" => {
                rtp.site_setup = site_setup::parse(&params)?;
            },
            "SIM_DEF" => {
                rtp.simulation = simulation::parse(&params)?;
            },
            "FIELD_DEF" => {
                rtp.fields.push(field::parse(&params)?);
            },
            "EXTENDED_FIELD_DEF" => {
                rtp.extended_fields.push(extended_field::parse(&params)?);
            }
            _ => {}
        }
    }
    Ok(rtp)
}

/// Parse a line from an RTP file into separate parameters.
///
/// # Arguments
///
/// * `line`: expected format: "param1","param2","param3"
///
/// returns: Vec<String, Global>
pub(crate) fn get_params(line: &str) -> Vec<String> {
    let n = line.len();
    if n == 0 {
        return vec![];
    }
    if line.chars().nth(0).unwrap() != '"' || line.chars().nth(n - 1).unwrap() != '"' {
        return vec![];
    }
    let mut index = 0;
    let mut v = vec![];
    let pattern = "\",\"";
    for (i, _) in line.match_indices(pattern) {
        let param = line[index + 1..i].to_string();
        v.push(param);
        index = i + pattern.len() - 1;
    }
    if index < n - 2 {
        let param = line[index + 1..n - 1].to_string();
        v.push(param);
    }
    return v;
}

#[cfg(test)]
mod tests {
    #[test]
    fn line_to_params_empty() {
        let v = super::get_params("");
        assert!(v.is_empty());
    }

    #[test]
    fn line_to_params_one_param() {
        let v = super::get_params("\"param\"");
        assert_eq!(v.len(), 1);
        assert_eq!("param", v[0]);
    }

    #[test]
    fn line_to_params() {
        let v = super::get_params("\"param1\",\"param2\",\"param3\"");
        assert_eq!(v.len(), 3);
        assert_eq!("param1", v[0]);
        assert_eq!("param2", v[1]);
        assert_eq!("param3", v[2]);
    }
}