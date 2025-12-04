use walkdir::WalkDir;

use crate::{db::entities::Track, flog_ron, scanner::scan_file::ScannedFile};

#[derive(Clone)]
pub struct LibraryScanner {
    library_path: String,
}

impl LibraryScanner {
    pub fn new(library_path: String) -> Self {
        Self { library_path }
    }
}

impl LibraryScanner {
    pub fn scan_library(&self) -> Result<(), anyhow::Error> {
        for entry in WalkDir::new(&self.library_path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                let scanned_file = ScannedFile::scan(&path)?;
                let track: Track = scanned_file.into();
                flog_ron!(track);
            }
        }
        Ok(())
    }
}
