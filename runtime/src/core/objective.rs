use std::fmt;

/// An objective bound to an APS execution.
///
/// The objective describes what the execution is trying to accomplish.
///
/// Once constructed, the objective exposes read access only.
/// No mutation API is provided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Objective {
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectiveError {
    Empty,
}

impl Objective {
    /// Creates a validated objective.
    pub fn new(description: impl Into<String>) -> Result<Self, ObjectiveError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ObjectiveError::Empty);
        }

        Ok(Self { description })
    }

    /// Returns the objective description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for Objective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_valid_objective() {
        let objective =
            Objective::new("Solve the programming task correctly").unwrap();

        assert_eq!(
            objective.description(),
            "Solve the programming task correctly"
        );
    }

    #[test]
    fn rejects_empty_objective() {
        assert_eq!(
            Objective::new(""),
            Err(ObjectiveError::Empty)
        );
    }

    #[test]
    fn rejects_whitespace_only_objective() {
        assert_eq!(
            Objective::new("     "),
            Err(ObjectiveError::Empty)
        );
    }
}