use std::path::Path;

mod error;
mod extension;
mod structure;

pub use error::ValidationError;
pub use extension::ExtensionValidator;
pub use structure::ArchiveStructureValidator;

#[derive(Clone)]
pub struct ArchiveValidator {
    ext_validator: ExtensionValidator,
}

impl ArchiveValidator {
    pub fn new(allowed_extensions: Vec<String>) -> Self {
        Self {
            ext_validator: ExtensionValidator::new(allowed_extensions),
        }
    }

    pub fn validate_extension(&self, path: &Path) -> Result<(), ValidationError> {
        self.ext_validator.validate(path)
    }

    pub fn validate_structure(&self, path: &Path) -> Result<(), ValidationError> {
        ArchiveStructureValidator::validate(path)
    }

    pub fn validate(&self, archive_path: &Path) -> Result<(), ValidationError> {
        self.ext_validator.validate(archive_path)?;
        ArchiveStructureValidator::validate(archive_path)?;
        Ok(())
    }
}
