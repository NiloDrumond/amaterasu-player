use ffmpeg_next::format::context::Input;
use sha2::{Digest, Sha256};

use crate::scanner::error::ScannerResult;
use crate::scanner::scan_cover::ScannedCover;

pub struct PacketScanOutput {
    pub audio_hash: [u8; 32],
    pub cover: Option<ScannedCover>,
}

pub fn scan_packets(
    mut ictx: Input,
    audio_stream_index: usize,
    cover_stream_index: Option<usize>,
) -> ScannerResult<PacketScanOutput> {
    let mut hasher = Sha256::new();
    let mut cover_bytes: Option<Vec<u8>> = None;

    for (stream, packet) in ictx.packets() {
        let idx = stream.index();
        if idx == audio_stream_index {
            if let Some(data) = packet.data() {
                hasher.update(data);
            }
        } else if cover_bytes.is_none() && Some(idx) == cover_stream_index {
            if let Some(data) = packet.data() {
                cover_bytes = Some(data.to_vec());
            }
        }
    }

    let cover = cover_bytes.and_then(|bytes| {
        let ext = infer::get(&bytes)?.extension();
        let hash: [u8; 32] = Sha256::digest(&bytes).into();
        Some(ScannedCover { bytes, hash, ext })
    });

    Ok(PacketScanOutput {
        audio_hash: hasher.finalize().into(),
        cover,
    })
}
