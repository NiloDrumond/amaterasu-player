const LOSSLESS_EXTENSIONS: &[&str] = &["flac", "alac", "ape", "wv", "wav", "aiff", "aif"];

pub fn is_lossless(format: &str) -> bool {
    LOSSLESS_EXTENSIONS.contains(&format)
}

pub fn quality_rank(
    format: &str,
    bitrate: Option<i32>,
    sample_rate: Option<i64>,
) -> (u8, i32, i64) {
    let tier: u8 = if is_lossless(format) { 1 } else { 0 };
    (tier, bitrate.unwrap_or(0), sample_rate.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lossless_beats_lossy_regardless_of_bitrate() {
        let flac = quality_rank("flac", Some(900_000), Some(44_100));
        let mp3_320 = quality_rank("mp3", Some(320_000), Some(44_100));
        assert!(flac > mp3_320);
    }

    #[test]
    fn within_lossless_higher_sample_rate_wins_when_bitrate_equal() {
        let hi = quality_rank("flac", Some(1_000_000), Some(96_000));
        let lo = quality_rank("flac", Some(1_000_000), Some(44_100));
        assert!(hi > lo);
    }

    #[test]
    fn within_lossy_higher_bitrate_wins() {
        let hi = quality_rank("mp3", Some(320_000), Some(44_100));
        let lo = quality_rank("mp3", Some(128_000), Some(44_100));
        assert!(hi > lo);
    }

    #[test]
    fn unknown_extension_is_lossy() {
        assert!(!is_lossless("xyz"));
        let flac = quality_rank("flac", None, None);
        let unknown = quality_rank("xyz", Some(10_000_000), Some(192_000));
        assert!(flac > unknown);
    }

    #[test]
    fn missing_bitrate_and_sample_rate_default_to_zero() {
        let flac_none = quality_rank("flac", None, None);
        let mp3_320 = quality_rank("mp3", Some(320_000), Some(44_100));
        assert!(flac_none > mp3_320);
    }
}
