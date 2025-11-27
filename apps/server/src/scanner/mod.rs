mod error;
mod file_walker;

use symphonia::core::{
    codecs::CODEC_TYPE_NULL, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint
};
use walkdir::WalkDir;

use crate::scanner::error::{ScannerError, ScannerResult};

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
    fn scan_file(&self, path: &std::path::Path) -> ScannerResult<()> {
        let probe = symphonia::default::get_probe();
        let src = std::fs::File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {
                hint.with_extension(extension);
            }
        }
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = probe.format(&hint, mss, &fmt_opts, &meta_opts)?;
        let format = probed.format;
        let mut metadata = probed.metadata;
        let metadata = metadata.get();
        let Some(mut metadata) = metadata else {
            return Err(ScannerError::FailedToExtractMetadata);
        };
        let Some(metadata) = metadata.skip_to_latest() else {
            return Err(ScannerError::FailedToExtractMetadata);
        };

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL).ok_or(ScannerError::FailedToDetectFormat)?;

        let codec = track.codec_params.codec;

        println!("Metadata: {:?}", metadata.tags());

        Ok(())
    }

    pub fn scan_library(&self) -> Result<(), anyhow::Error> {
        for entry in WalkDir::new(&self.library_path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                self.scan_file(path)?;
            }
            entry.path();

            println!("{:?}", entry);
        }
        Ok(())
    }
}
