use std::fmt;

/// Represents parsed parts of the structured code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredCode {
    /// category: 3 digits (e.g., 200, 400, 500)
    pub category: i32,
    /// system: 4 digits
    pub system: i32,
    /// detail: 4 digits
    pub detail: i32,
}

#[derive(Debug, thiserror::Error)]
pub enum CodeError {
    #[error("invalid length: expected 3,4 or 11 digits")] 
    InvalidLength,
    #[error("invalid numeric value")] 
    InvalidNumber,
    #[error("invalid category: {0}")] 
    InvalidCategory(i32),
}

impl fmt::Display for StructuredCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // produce full 11-digit code as i64-safe string
        write!(f, "{category:03}{system:04}{detail:04}", category=self.category, system=self.system, detail=self.detail)
    }
}

/// Generate an 11-digit code from parts.
///
/// - `category` should be 200, 400, 500 (or compatible)
/// - `system` and `detail` are 0..=9999
pub fn make_code(category: i32, system: i32, detail: i32) -> Result<i64, CodeError> {
    if category != 200 && category != 400 && category != 500 {
        return Err(CodeError::InvalidCategory(category));
    }
    if !(0..=9999).contains(&system) || !(0..=9999).contains(&detail) {
        return Err(CodeError::InvalidNumber);
    }
    // safe to compute as i64
    // category occupies the highest 3 digits of an 11-digit number -> multiplier 10^8
    let code = (category as i64) * 100_000_000 + (system as i64) * 10_000 + (detail as i64);
    Ok(code)
}

/// Parse an integer code into StructuredCode. Accepts either 200/400/500 (short) or full 11-digit codes.
pub fn parse_code(code: i64) -> Result<StructuredCode, CodeError> {
    if code == 200 || code == 400 || code == 500 {
        return Ok(StructuredCode { category: code as i32, system: 0, detail: 0 });
    }
    if code < 0 {
        return Err(CodeError::InvalidNumber);
    }
    // Extract parts from 11-digit representation
    let category = (code / 100_000_000) as i32;
    let system = ((code / 10_000) % 10_000) as i32;
    let detail = (code % 10_000) as i32;
    if !(category == 200 || category == 400 || category == 500) {
        return Err(CodeError::InvalidCategory(category));
    }
    Ok(StructuredCode { category, system, detail })
}

/// Validate a code (short or structured) for basic semantic rules.
pub fn is_valid_code(code: i64) -> bool {
    match parse_code(code) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Common error categories as enum. Values are short-form categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Ok = 200,
    BadRequest = 400,
    Internal = 500,
}

impl ErrorCode {
    /// numeric value
    pub fn value(self) -> i64 {
        self as i64
    }

    /// convenience: produce StructuredCode with zeroed system/detail
    pub fn as_structured(self) -> StructuredCode {
        StructuredCode { category: self as i32, system: 0, detail: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_and_parse() {
        let c = make_code(400, 1001, 1).unwrap();
        assert_eq!(c, 40010010001i64);
        let parsed = parse_code(c).unwrap();
        assert_eq!(parsed.category, 400);
        assert_eq!(parsed.system, 1001);
        assert_eq!(parsed.detail, 1);
    }

    #[test]
    fn short_codes() {
        let s = parse_code(200).unwrap();
        assert_eq!(s.category, 200);
        assert_eq!(s.system, 0);
        assert_eq!(s.detail, 0);
    }

    #[test]
    fn invalid_category() {
        let r = make_code(201, 1, 1);
        assert!(r.is_err());
        let p = parse_code(201i64);
        assert!(p.is_err());
    }

    #[test]
    fn invalid_parts() {
        assert!(make_code(400, 10_000, 0).is_err());
        assert!(!is_valid_code(-1));
    }
}
