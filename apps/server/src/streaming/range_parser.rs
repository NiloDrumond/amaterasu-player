#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RangeError {
    #[error("malformed range header")]
    Malformed,
    #[error("range not satisfiable")]
    NotSatisfiable,
}

/// Parses a `Range` header for a known `total_len`. Returns
/// `(start, end_inclusive, total_len)` per RFC 7233.
///
/// Supports single ranges only: `bytes=N-M`, `bytes=N-`, `bytes=-N`.
/// Multipart ranges and non-`bytes` units are rejected as `Malformed`.
pub fn parse_range(header: &str, total_len: u64) -> Result<(u64, u64, u64), RangeError> {
    let rest = header
        .strip_prefix("bytes=")
        .ok_or(RangeError::Malformed)?
        .trim();

    if rest.contains(',') {
        return Err(RangeError::Malformed);
    }

    let (from, to) = rest.split_once('-').ok_or(RangeError::Malformed)?;
    let from = from.trim();
    let to = to.trim();

    let (start, end) = match (from.is_empty(), to.is_empty()) {
        (true, true) => return Err(RangeError::Malformed),
        (true, false) => {
            // Suffix: last N bytes.
            let n: u64 = to.parse().map_err(|_| RangeError::Malformed)?;
            if total_len == 0 || n == 0 {
                return Err(RangeError::NotSatisfiable);
            }
            let start = total_len.saturating_sub(n);
            (start, total_len - 1)
        }
        (false, true) => {
            let start: u64 = from.parse().map_err(|_| RangeError::Malformed)?;
            if total_len == 0 || start >= total_len {
                return Err(RangeError::NotSatisfiable);
            }
            (start, total_len - 1)
        }
        (false, false) => {
            let start: u64 = from.parse().map_err(|_| RangeError::Malformed)?;
            let end: u64 = to.parse().map_err(|_| RangeError::Malformed)?;
            if total_len == 0 || start >= total_len || start > end {
                return Err(RangeError::NotSatisfiable);
            }
            (start, end.min(total_len - 1))
        }
    };

    Ok((start, end, total_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_range() {
        assert_eq!(parse_range("bytes=0-1023", 2048), Ok((0, 1023, 2048)));
    }

    #[test]
    fn closed_range_clamps_end() {
        assert_eq!(parse_range("bytes=100-9999", 200), Ok((100, 199, 200)));
    }

    #[test]
    fn open_ended_range() {
        assert_eq!(parse_range("bytes=500-", 1000), Ok((500, 999, 1000)));
    }

    #[test]
    fn suffix_range() {
        assert_eq!(parse_range("bytes=-256", 1000), Ok((744, 999, 1000)));
    }

    #[test]
    fn suffix_larger_than_total() {
        assert_eq!(parse_range("bytes=-5000", 1000), Ok((0, 999, 1000)));
    }

    #[test]
    fn start_past_end_not_satisfiable() {
        assert_eq!(
            parse_range("bytes=99999-", 1000),
            Err(RangeError::NotSatisfiable)
        );
    }

    #[test]
    fn start_greater_than_end_not_satisfiable() {
        assert_eq!(
            parse_range("bytes=500-400", 1000),
            Err(RangeError::NotSatisfiable)
        );
    }

    #[test]
    fn empty_total_not_satisfiable() {
        assert_eq!(parse_range("bytes=0-0", 0), Err(RangeError::NotSatisfiable));
    }

    #[test]
    fn multipart_rejected() {
        assert_eq!(parse_range("bytes=0-0,2-2", 10), Err(RangeError::Malformed));
    }

    #[test]
    fn wrong_unit_rejected() {
        assert_eq!(parse_range("items=0-10", 100), Err(RangeError::Malformed));
    }

    #[test]
    fn both_empty_rejected() {
        assert_eq!(parse_range("bytes=-", 100), Err(RangeError::Malformed));
    }

    #[test]
    fn non_numeric_rejected() {
        assert_eq!(parse_range("bytes=a-b", 100), Err(RangeError::Malformed));
    }
}
