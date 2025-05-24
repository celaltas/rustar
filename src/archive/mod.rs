mod appender;
mod builder;
mod error;
mod extractor;
mod lister;

use std::path::Path;

pub use appender::ArchiveAppender;
pub use builder::ArchiveBuilder;
pub use error::ArchiverError;
pub use extractor::ArchiveExtractor;
use lister::ArchiveEntry;
pub use lister::ArchiveLister;

use crate::validation::ArchiveValidator;

pub struct Archiver {
    builder: ArchiveBuilder,
    extractor: ArchiveExtractor,
    lister: ArchiveLister,
    appender: ArchiveAppender,
    validator: ArchiveValidator,
}

impl Archiver {
    pub fn new(allowed_extensions: Vec<String>) -> Self {
        let validator = ArchiveValidator::new(allowed_extensions);
        Self {
            builder: ArchiveBuilder::new(),
            extractor: ArchiveExtractor::new(validator.clone()),
            lister: ArchiveLister::new(validator.clone()),
            appender: ArchiveAppender::new(validator.clone()),
            validator,
        }
    }

    // Builder methods
    pub fn create(
        &self,
        archive_path: &Path,
        files: Vec<impl AsRef<Path>>,
    ) -> Result<(), ArchiverError> {
        self.validator.validate_extension(archive_path)?;
        self.builder.build(archive_path, files)
    }

    // Extractor methods
    pub fn extract(&self, archive_path: &Path, output_dir: &Path) -> Result<(), ArchiverError> {
        self.extractor.extract(archive_path, output_dir)
    }

    // Lister methods
    pub fn list(&self, archive_path: &Path) -> Result<Vec<ArchiveEntry>, ArchiverError> {
        self.lister.list(archive_path)
    }

    // Appender methods
    pub fn append(
        &self,
        archive_path: &Path,
        files: Vec<impl AsRef<Path>>,
    ) -> Result<(), ArchiverError> {
        self.appender.append(archive_path, files)
    }
}
