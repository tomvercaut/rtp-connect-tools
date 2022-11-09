use crate::io::{Error, opt_f64, opt_u32};
use crate::model::MultiLeafCollimator;

#[repr(usize)]
#[derive(Debug, Copy, Clone)]
enum Id {
    Keyword = 0,
    FieldId,
    MlcType,
    MlcLeaves,
    MlcA,
    MlcB = Id::MlcA as usize + 50,
    Crc = Id::MlcB as usize + 50,
}

/// Parse the data elements from a Multi Leaf Collimator from a set of RTP parameters.
///
/// # Arguments
///
/// * `params`: multi leaf collimator definition record parameters
///
/// returns: Result<MultiLeafCollimator, Error>
pub fn parse(params: &Vec<String>) -> Result<MultiLeafCollimator, Error> {
    let mut mlc = MultiLeafCollimator::default();
    let expected_length = 105;
    let expected_keyword = "MLC_DEF";
    let max_leaves_bank = 50u32;
    if params.len() != expected_length {
        return Err(Error::ParameterLengthMismatch(expected_length, params.len()));
    }
    if params[Id::Keyword as usize] != expected_keyword {
        return Err(Error::Keyword(expected_keyword.to_string(), params[0].to_string()));
    }
    mlc.field_id = params[Id::FieldId as usize].to_string();
    mlc.mlc_type = params[Id::MlcType as usize].to_string();
    mlc.mlc_leaves = opt_u32(params[Id::MlcLeaves as usize].as_str()).unwrap();
    for i in 0..max_leaves_bank {
        if i < mlc.mlc_leaves {
            let leaf = opt_f64(params[Id::MlcA as usize + i as usize].as_str()).unwrap();
            mlc.mlc_a.push(leaf);
        }
    }
    for i in 0..max_leaves_bank {
        if i < mlc.mlc_leaves {
            let leaf = opt_f64(params[Id::MlcB as usize + i as usize].as_str()).unwrap();
            mlc.mlc_b.push(leaf);
        }
    }
    mlc.crc = (&params[Id::Crc as usize]).parse::<i32>()?;
    Ok(mlc)
}


#[cfg(test)]
mod tests {
    use crate::io::multi_leaf_collimator::Id;

    #[test]
    fn parse() {
        let leaf_count = 40u32;
        let max_leaves = 50u32;
        let mut params = vec![
            "MLC_DEF".to_string(),
            "FieldId".to_string(),
            "MlcType".to_string(),
            leaf_count.to_string(),
        ];
        for i in 0..leaf_count {
            params.push((Id::MlcA as usize + i as usize).to_string());
        }
        for _i in leaf_count..max_leaves {
            params.push("".to_string());
        }
        for i in 0..leaf_count {
            params.push((Id::MlcB as usize + i as usize).to_string());
        }
        for _i in leaf_count..max_leaves {
            params.push("".to_string());
        }
        params.push("1234".to_string());
        let res = super::parse(&params);
        if res.is_err() {
            eprintln!("{:#?}", res.as_ref().err());
        }
        assert!(res.is_ok());
        let mlc = res.unwrap();
        assert_eq!(&mlc.field_id, "FieldId");
        assert_eq!(&mlc.mlc_type, "MlcType");
        assert_eq!(mlc.mlc_leaves, leaf_count);
        assert_eq!(mlc.mlc_a.len(), leaf_count as usize);
        assert_eq!(mlc.mlc_b.len(), leaf_count as usize);
        for i in 0..leaf_count {
            assert_eq!(mlc.mlc_a[i as usize], (Id::MlcA as u32 + i) as f64);
            assert_eq!(mlc.mlc_b[i as usize], (Id::MlcB as u32 + i) as f64);
        }
        assert_eq!(mlc.crc, 1234);
    }
}