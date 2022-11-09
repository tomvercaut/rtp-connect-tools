mod parser;
pub(crate) mod plan;
pub(crate) mod extended_plan;
pub(crate) mod prescription;
pub(crate) mod site_setup;
pub(crate) mod simulation;
pub(crate) mod field;
pub(crate) mod extended_field;
pub(crate) mod document_based_treatment_field;

pub use parser::*;


/// Convert a string to an optional u32. If the string is empty or if a parsing error occurs,
/// None is returned.
///
/// # Arguments
///
/// * `s`: numeric string
///
/// returns: Option<f64>
pub(crate) fn opt_u32(s: &str) -> Option<u32> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<u32>().ok()
}

/// Convert a string to an optional f64. If the string is empty or if a parsing error occurs,
/// None is returned.
///
/// # Arguments
///
/// * `s`: numeric string
///
/// returns: Option<f64>
pub(crate) fn opt_f64(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<f64>().ok()
}


#[cfg(test)]
mod tests {
    #[test]
    fn opt_u32() {
        assert_eq!(Some(5u32), super::opt_u32(" 5 "));
        assert_eq!(None, super::opt_u32(" -5 "));
        assert_eq!(None, super::opt_u32("  "));
        assert_eq!(None, super::opt_u32(" abc "));
    }

    #[test]
    fn opt_f64() {
        assert_eq!(Some(5.0), super::opt_f64(" 5.0 "));
        assert_eq!(Some(-5.0), super::opt_f64(" -5.0 "));
        assert_eq!(Some(5f64), super::opt_f64(" 5 "));
        assert_eq!(None, super::opt_f64("  "));
        assert_eq!(None, super::opt_f64(" abc "));
    }
}