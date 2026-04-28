use ffmpeg_next::format::context::Input;
use ffmpeg_next::format::stream::Disposition;

pub struct ScannedCover {
    pub bytes: Vec<u8>,
    pub hash: [u8; 32],
    pub ext: &'static str,
}

impl ScannedCover {
    pub fn stream_index(ictx: &Input) -> Option<usize> {
        ictx.streams()
            .find(|s| s.disposition().contains(Disposition::ATTACHED_PIC))
            .map(|s| s.index())
    }
}
