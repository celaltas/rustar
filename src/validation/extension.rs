use std::path::Path;

use super::error::ValidationError;

#[derive(Clone)]
pub struct ExtensionValidator {
    allowed: Vec<String>,
}

impl ExtensionValidator {
    pub fn new(allowed: Vec<String>) -> Self {
        Self { allowed }
    }

    pub fn validate(&self, path: &Path) -> Result<(), ValidationError> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !self.allowed.iter().any(|a| a.to_lowercase() == ext) {
            Err(ValidationError::InvalidExtension(format!(
                "Extension '{}' not allowed. Allowed: {:?}",
                ext, self.allowed
            )))
        } else {
            Ok(())
        }
    }
}
