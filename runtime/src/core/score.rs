use std::fmt;

/// A validated outcome-quality score used by the APS trusted runtime.
///
/// # Invariant
///
/// The contained value is finite and lies within the closed interval
/// `[0.0, 1.0]`.  
/*
Rustdoc found:

0.0 <= Q <= 1.0

inside our documentation and effectively attempted to compile it
 */
///
/// NaN and positive/negative infinity are rejected.
///
/// Once a `Score` exists, runtime components may rely on this
/// invariant without independently validating the underlying value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Score(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScoreError {
    NotFinite,
    OutOfRange(f64),
}

impl Score {
    /// Constructs a validated score.
    pub fn new(value: f64) -> Result<Self, ScoreError> {
        if !value.is_finite() {
            return Err(ScoreError::NotFinite);
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(ScoreError::OutOfRange(value));
        }

        Ok(Self(value))
    }

    /// Returns the validated numeric value.
    pub fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_lower_boundary() {
        let score = Score::new(0.0).unwrap();

        assert_eq!(score.value(), 0.0);
    }

    #[test]
    fn accepts_upper_boundary() {
        let score = Score::new(1.0).unwrap();

        assert_eq!(score.value(), 1.0);
    }

    #[test]
    fn accepts_value_inside_range() {
        let score = Score::new(0.73).unwrap();

        assert_eq!(score.value(), 0.73);
    }

    #[test]
    fn rejects_negative_value() {
        assert_eq!(
            Score::new(-0.1),
            Err(ScoreError::OutOfRange(-0.1))
        );
    }

    #[test]
    fn rejects_value_above_one() {
        assert_eq!(
            Score::new(1.01),
            Err(ScoreError::OutOfRange(1.01))
        );
    }

    #[test]
    fn rejects_nan() {
        assert_eq!(
            Score::new(f64::NAN),
            Err(ScoreError::NotFinite)
        );
    }

    #[test]
    fn rejects_positive_infinity() {
        assert_eq!(
            Score::new(f64::INFINITY),
            Err(ScoreError::NotFinite)
        );
    }

    #[test]
    fn rejects_negative_infinity() {
        assert_eq!(
            Score::new(f64::NEG_INFINITY),
            Err(ScoreError::NotFinite)
        );
    }
}