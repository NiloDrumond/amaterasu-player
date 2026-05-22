//! Pure transformations from MB JSON shapes into the canonical `proposed`
//! payload stored on `metadata_suggestions.proposed`. Kept HTTP-free so it's
//! trivially testable.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::models::{Artist as MbArtist, ArtistCredit, Recording, ReleaseGroup, ReleaseSummary};

/// What we propose to write onto an `Album` entity if the admin accepts this
/// candidate. Optional fields = no suggestion for that column; the admin
/// shouldn't see a "blank → blank" diff.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumProposal {
    pub mbid: String,
    pub release_group_mbid: String,
    pub title: Option<String>,
    pub sort_title: Option<String>,
    pub date: Option<NaiveDate>,
    pub artist_mbid: Option<String>,
    pub artist_name: Option<String>,
    /// MB release-group search doesn't include child releases by default, so
    /// these are typically empty. CAA serves covers by release-group MBID
    /// directly, so we don't need a specific release MBID to fetch art.
    pub primary_release_mbid: Option<String>,
    pub primary_release_country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistProposal {
    pub mbid: String,
    pub name: Option<String>,
    pub sort_name: Option<String>,
    pub country: Option<String>,
    pub disambiguation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackProposal {
    pub mbid: String,
    pub title: Option<String>,
    pub artist_mbid: Option<String>,
    pub artist_name: Option<String>,
    pub release_mbid: Option<String>,
    pub release_title: Option<String>,
    pub length_ms: Option<i64>,
}

/// A single candidate as we'll persist it. `mbid` is the entity-level MBID
/// stamped on the parent entity when accepted (release-group MBID for albums,
/// artist MBID for artists, recording MBID for tracks).
#[derive(Debug, Clone)]
pub struct AlbumCandidate {
    pub mbid: String,
    pub score: i16,
    pub proposal: AlbumProposal,
}

#[derive(Debug, Clone)]
pub struct ArtistCandidate {
    pub mbid: String,
    pub score: i16,
    pub proposal: ArtistProposal,
}

#[derive(Debug, Clone)]
pub struct TrackCandidate {
    pub mbid: String,
    pub score: i16,
    pub proposal: TrackProposal,
}

pub fn release_group_to_album_candidate(rg: &ReleaseGroup) -> Option<AlbumCandidate> {
    if rg.id.is_empty() {
        return None;
    }
    let (artist_mbid, artist_name) = first_artist_credit(&rg.artist_credit);
    // MB's release-group search endpoint usually omits child releases (you'd
    // need a separate /release/?release-group={mbid} call to get them), so we
    // store the primary release as Option and fall back to the release-group
    // MBID when fetching cover art from CAA.
    let primary = pick_primary_release(&rg.releases);
    Some(AlbumCandidate {
        mbid: rg.id.clone(),
        score: clamp_score(rg.score),
        proposal: AlbumProposal {
            mbid: rg.id.clone(),
            release_group_mbid: rg.id.clone(),
            title: non_empty(&rg.title),
            sort_title: None, // MB doesn't return sort title for release-group search
            date: rg
                .first_release_date
                .as_deref()
                .and_then(parse_partial_date),
            artist_mbid,
            artist_name,
            primary_release_mbid: primary.map(|r| r.id.clone()),
            primary_release_country: primary.and_then(|r| r.country.clone()),
        },
    })
}

pub fn artist_to_candidate(a: &MbArtist) -> ArtistCandidate {
    ArtistCandidate {
        mbid: a.id.clone(),
        score: clamp_score(a.score),
        proposal: ArtistProposal {
            mbid: a.id.clone(),
            name: non_empty(&a.name),
            sort_name: a.sort_name.clone(),
            country: a.country.clone(),
            disambiguation: a.disambiguation.clone(),
        },
    }
}

pub fn recording_to_candidate(r: &Recording) -> TrackCandidate {
    let (artist_mbid, artist_name) = first_artist_credit(&r.artist_credit);
    let (release_mbid, release_title) = r
        .releases
        .first()
        .map(|rel| (Some(rel.id.clone()), non_empty(&rel.title)))
        .unwrap_or((None, None));
    TrackCandidate {
        mbid: r.id.clone(),
        score: clamp_score(r.score),
        proposal: TrackProposal {
            mbid: r.id.clone(),
            title: non_empty(&r.title),
            artist_mbid,
            artist_name,
            release_mbid,
            release_title,
            length_ms: r.length,
        },
    }
}

fn pick_primary_release(releases: &[ReleaseSummary]) -> Option<&ReleaseSummary> {
    // Prefer a release with a country code (more canonical) and a date, then
    // any release, then nothing.
    releases
        .iter()
        .find(|r| r.country.is_some() && r.date.is_some())
        .or_else(|| releases.iter().find(|r| r.country.is_some()))
        .or_else(|| releases.first())
}

fn first_artist_credit(credits: &[ArtistCredit]) -> (Option<String>, Option<String>) {
    if credits.is_empty() {
        return (None, None);
    }
    // Reconstruct the displayed name from the credit joinphrases so collab
    // releases come through as "A & B" etc.
    let mut name = String::new();
    for c in credits {
        if !c.name.is_empty() {
            name.push_str(&c.name);
        } else if let Some(a) = &c.artist {
            name.push_str(&a.name);
        }
        if let Some(j) = &c.joinphrase {
            name.push_str(j);
        }
    }
    let first_mbid = credits
        .iter()
        .find_map(|c| c.artist.as_ref().map(|a| a.id.clone()));
    (first_mbid, non_empty(&name))
}

fn non_empty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

fn clamp_score(score: i32) -> i16 {
    score.clamp(0, 100) as i16
}

/// MB returns dates as `YYYY`, `YYYY-MM`, or `YYYY-MM-DD`. We pad partial
/// dates to month/day = 1 so they round-trip through `NaiveDate`.
fn parse_partial_date(raw: &str) -> Option<NaiveDate> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let parts: Vec<&str> = raw.split('-').collect();
    let y: i32 = parts.first().and_then(|s| s.parse().ok())?;
    let m: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let d: u32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    NaiveDate::from_ymd_opt(y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_partial_and_year_only_dates() {
        assert_eq!(
            parse_partial_date("1973-03-01"),
            Some(NaiveDate::from_ymd_opt(1973, 3, 1).unwrap())
        );
        assert_eq!(
            parse_partial_date("1973-03"),
            Some(NaiveDate::from_ymd_opt(1973, 3, 1).unwrap())
        );
        assert_eq!(
            parse_partial_date("1973"),
            Some(NaiveDate::from_ymd_opt(1973, 1, 1).unwrap())
        );
        assert_eq!(parse_partial_date(""), None);
        assert_eq!(parse_partial_date("not-a-date"), None);
    }

    #[test]
    fn maps_release_group_with_release_to_album_candidate() {
        let rg = ReleaseGroup {
            id: "rg-1".into(),
            title: "Dark Side of the Moon".into(),
            score: 98,
            primary_type: Some("Album".into()),
            first_release_date: Some("1973-03-01".into()),
            artist_credit: vec![ArtistCredit {
                name: "Pink Floyd".into(),
                joinphrase: None,
                artist: Some(super::super::models::ArtistRef {
                    id: "artist-1".into(),
                    name: "Pink Floyd".into(),
                    sort_name: Some("Floyd, Pink".into()),
                }),
            }],
            releases: vec![ReleaseSummary {
                id: "rel-1".into(),
                title: "Dark Side of the Moon".into(),
                date: Some("1973-03-01".into()),
                country: Some("GB".into()),
            }],
        };

        let c = release_group_to_album_candidate(&rg).expect("candidate built");
        assert_eq!(c.score, 98);
        assert_eq!(c.proposal.title.as_deref(), Some("Dark Side of the Moon"));
        assert_eq!(c.proposal.artist_name.as_deref(), Some("Pink Floyd"));
        assert_eq!(c.proposal.artist_mbid.as_deref(), Some("artist-1"));
        assert_eq!(c.proposal.primary_release_mbid.as_deref(), Some("rel-1"));
        assert_eq!(c.proposal.primary_release_country.as_deref(), Some("GB"));
        assert_eq!(
            c.proposal.date,
            Some(NaiveDate::from_ymd_opt(1973, 3, 1).unwrap())
        );
    }

    #[test]
    fn release_group_without_child_releases_still_produces_candidate() {
        // MB's release-group search endpoint usually omits child releases.
        // We must still produce a candidate so the admin can review the
        // match -- CAA covers can be fetched by release-group MBID.
        let rg = ReleaseGroup {
            id: "rg-2".into(),
            title: "Phantom".into(),
            score: 50,
            primary_type: None,
            first_release_date: None,
            artist_credit: vec![],
            releases: vec![],
        };
        let c = release_group_to_album_candidate(&rg).expect("candidate built");
        assert_eq!(c.proposal.primary_release_mbid, None);
    }

    #[test]
    fn release_group_with_empty_id_is_rejected() {
        let rg = ReleaseGroup {
            id: String::new(),
            title: "x".into(),
            score: 0,
            primary_type: None,
            first_release_date: None,
            artist_credit: vec![],
            releases: vec![],
        };
        assert!(release_group_to_album_candidate(&rg).is_none());
    }

    #[test]
    fn artist_credit_joins_collaborations() {
        let credits = vec![
            ArtistCredit {
                name: "Bowie".into(),
                joinphrase: Some(" & ".into()),
                artist: None,
            },
            ArtistCredit {
                name: "Queen".into(),
                joinphrase: None,
                artist: None,
            },
        ];
        let (_, name) = first_artist_credit(&credits);
        assert_eq!(name.as_deref(), Some("Bowie & Queen"));
    }
}
