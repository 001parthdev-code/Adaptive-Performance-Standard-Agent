use std::fmt;

/// A validated objective used by the APS trusted runtime.
///
/// An `Objective` owns its description and exposes no public
/// mutation API.
///
/// This protects the contents of an objective after construction.
/// Execution-level objective replacement is enforced separately
/// by the execution context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Objective {
    description: String,
}

/// Errors that may occur while constructing an [`Objective`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectiveError {
    /// The supplied objective contains no non-whitespace content.
    Empty,
}

impl Objective {
    /// Creates a validated objective.
    ///
    /// Empty and whitespace-only descriptions are rejected.
    pub fn new(description: impl Into<String>) -> Result<Self, ObjectiveError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(ObjectiveError::Empty);
        }

        Ok(Self { description })
    }

    /// Returns a read-only view of the objective description.
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
    fn rejects_spaces_only_objective() {
        assert_eq!(
            Objective::new("     "),
            Err(ObjectiveError::Empty)
        );
    }

    #[test]
    fn rejects_other_whitespace_only_objectives() {
        assert_eq!(
            Objective::new("\t\n\r "),
            Err(ObjectiveError::Empty)
        );
    }

    #[test]
    fn preserves_original_valid_description() {
        let objective =
            Objective::new("  Solve the task correctly  ").unwrap();

        assert_eq!(
            objective.description(),
            "  Solve the task correctly  "
        );
    }
}