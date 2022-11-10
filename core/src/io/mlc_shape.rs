use crate::io::{Error, opt_f64, opt_u32};
use crate::io::mlc_shape::Id::XCoordinate1;
use crate::model::MlcShape;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    FieldId,
    ControlPtNumber,
    TotalShapePoints,
    XCoordinate1,
    YCoordinate1,
    Crc = XCoordinate1 as usize + 320,
}


/// Parse the data elements from an MLC Shape Record from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: MLC shape record parameters
///
/// returns: Result<MlcShape, Error>
pub fn parse(params: &Vec<String>) -> Result<MlcShape, Error> {
    let mut mlc = MlcShape::default();
    let expected_length = 325;
    let expected_keyword = "MLC_SHAPE_DEF";
    let n_mlc = 160usize;
    let mlc_stride = 2usize;

    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    mlc.field_id = params[Id::FieldId as usize].to_string();
    mlc.control_pt_number = opt_u32(params[Id::ControlPtNumber as usize].as_str()).unwrap();
    mlc.total_shape_points = opt_u32(params[Id::TotalShapePoints as usize].as_str()).unwrap();
    for i in 0..n_mlc {
        if i < mlc.total_shape_points as usize {
            let x_i = XCoordinate1 as usize + i as usize * mlc_stride;
            let y_i = Id::YCoordinate1 as usize + i as usize * mlc_stride;
            mlc.coordinates_x.push(opt_f64(params[x_i].as_str()).unwrap());
            mlc.coordinates_y.push(opt_f64(params[y_i].as_str()).unwrap());
        }
    }
    mlc.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(mlc)
}

#[cfg(test)]
mod tests {
    use crate::io::mlc_shape::Id;

    #[test]
    fn parse() {
        let n_mlc = 160 * 2usize;
        let mut params = vec![
            "MLC_SHAPE_DEF".to_string(),
            "FieldId".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
        for i in 0..n_mlc {
            let value = Id::XCoordinate1 as usize + i;
            params.push(value.to_string());
        }

        params.push("1234".to_string());
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let mlc = res.unwrap();
        assert_eq!(&mlc.field_id, "FieldId");
        assert_eq!(mlc.control_pt_number, 3);
        assert_eq!(mlc.total_shape_points, 4);
        assert_eq!(mlc.total_shape_points as usize, mlc.coordinates_x.len());
        assert_eq!(mlc.total_shape_points as usize, mlc.coordinates_y.len());
        for i in 0..mlc.total_shape_points as usize {
            assert_eq!(mlc.coordinates_x[i], (Id::XCoordinate1 as usize + i * 2) as f64);
            assert_eq!(mlc.coordinates_y[i], (Id::YCoordinate1 as usize + i * 2) as f64);
        }
        assert_eq!(mlc.crc, 1234);
    }
}
