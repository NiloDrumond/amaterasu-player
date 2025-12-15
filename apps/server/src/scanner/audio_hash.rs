use ffmpeg_next::format::context::Input;
use sha2::{Digest, Sha256};

use crate::scanner::error::ScannerResult;

pub fn compute_audio_hash(mut ictx: Input, audio_stream_index: usize) -> ScannerResult<[u8; 32]> {
    let mut hasher = Sha256::new();

    for (stream, packet) in ictx.packets() {
        if stream.index() == audio_stream_index {
            if let Some(data) = packet.data() {
                hasher.update(data);
            }
        }
    }

    Ok(hasher.finalize().into())
}
