use std::str::FromStr;


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Rotation {
    // Clockwise
    CW,
    // Counterclockwise
    CC,
}


#[derive(thiserror::Error, Debug)]
pub enum RotationError {
    #[error("Rotation parsing error: {0}")]
    NoMatch(&'static str),
}

/// Test if a string is referencing a clockwise rotation.
///
/// # Arguments
///
/// * `value`: string equal to 'cw', 'CW' or 'clockwise' are considered a clockwise rotation
///
/// returns: bool
fn is_cw(value: &str) -> bool {
    return if value == "cw" || value == "CW" || value == "clockwise" {
        true
    } else {
        false
    }
}

/// Test if a string is referencing a counterclockwise rotation.
///
/// # Arguments
///
/// * `value`: string equal to 'cc', 'CC' or 'counterclockwise' are considered a counterclockwise rotation
///
/// returns: bool
fn is_cc(value: &str) -> bool {
    return if value == "cc" || value == "CC" || value == "counterclockwise" {
        true
    } else {
        false
    }
}

impl FromStr for Rotation {
    type Err = RotationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if is_cw(value) {
            return Ok(Rotation::CW);
        } else if is_cc(value) {
            return Ok(Rotation::CC);
        }
        Err(RotationError::NoMatch("no matching text found [case insensitive format: \
        cw || clockwise || cc || counterclockwise]"))
    }
}

impl<'a> TryFrom<&'a str> for Rotation {
    type Error = RotationError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if is_cw(value) {
            return Ok(Rotation::CW);
        } else if is_cc(value) {
            return Ok(Rotation::CC);
        }
        Err(RotationError::NoMatch("no matching text found [case insensitive format: \
        cw || clockwise || cc || counterclockwise]"))
    }
}

impl TryFrom<String> for Rotation {
    type Error = RotationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_cw(value.as_str()) {
            return Ok(Rotation::CW);
        } else if is_cc(value.as_str()) {
            return Ok(Rotation::CC);
        }
        Err(RotationError::NoMatch("no matching text found [case insensitive format: \
        cw || clockwise || cc || counterclockwise]"))
    }
}
