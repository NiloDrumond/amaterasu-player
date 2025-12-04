use walkdir::WalkDir;

use crate::scanner::scan_file::scan_file;

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
                scan_file(path)?;
            }
        }
        Ok(())
    }
}
