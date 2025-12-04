use sha2::{Digest, Sha256};
use symphonia::core::probe::ProbeResult;

use crate::scanner::error::{ScannerError, ScannerResult};

pub fn compute_audio_hash(probed: ProbeResult) -> ScannerResult<[u8; 32]> {
    let mut format = probed.format;
    let mut hasher = Sha256::new();

    let track_id = format
        .default_track()
        .ok_or_else(|| ScannerError::FailedToDetectDefaultTrack)?
        .id;

    loop {
        match format.next_packet() {
            Ok(packet) => {
                if packet.track_id() == track_id {
                    hasher.update(&packet.data);
                }
            }
            Err(symphonia::core::errors::Error::IoError(e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(hasher.finalize().into())
}
