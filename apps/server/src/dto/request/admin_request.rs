use chrono::NaiveDate;
use garde::Validate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tri-state field for partial-update DTOs:
/// - missing in JSON → `Unchanged`
/// - explicit `null` → `SetNull`
/// - any value → `Set(v)`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Patch<T> {
    #[default]
    Unchanged,
    SetNull,
    Set(T),
}

impl<T> Patch<T> {
    pub fn into_option_option(self) -> Option<Option<T>> {
        match self {
            Patch::Unchanged => None,
            Patch::SetNull => Some(None),
            Patch::Set(v) => Some(Some(v)),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Patch<T> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(d).map(|o| match o {
            Some(v) => Patch::Set(v),
            None => Patch::SetNull,
        })
    }
}

impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Patch::Unchanged => s.serialize_none(),
            Patch::SetNull => s.serialize_none(),
            Patch::Set(v) => v.serialize(s),
        }
    }
}

fn is_unchanged<T>(p: &Patch<T>) -> bool {
    matches!(p, Patch::Unchanged)
}

// NOTE: admin request DTOs are intentionally not exported to TypeScript via
// `api_type` because `Patch<T>`'s tri-state semantics don't map cleanly through
// `ts-rs`. Frontend mirrors these by hand in `apps/web/src/lib/admin/types.ts`.

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTrackParams {
    #[garde(length(min = 1, max = 1024))]
    pub title: Option<String>,
    #[garde(length(min = 0, max = 1024))]
    pub sort_title: Option<String>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub artist_id: Patch<Uuid>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub album_id: Patch<Uuid>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub disc: Patch<i32>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub track_no: Patch<i32>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub date: Patch<NaiveDate>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub composer: Patch<String>,
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "is_unchanged")]
    pub comment: Patch<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateTracksParams {
    #[garde(length(min = 1, max = 500))]
    pub ids: Vec<Uuid>,
    #[garde(dive)]
    pub patch: UpdateTrackParams,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpdateTracksResponse {
    pub updated: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlbumParams {
    #[garde(length(min = 1, max = 1024))]
    pub title: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_title: String,
    #[garde(skip)]
    pub artist_id: Option<Uuid>,
    #[garde(skip)]
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbumParams {
    #[garde(length(min = 1, max = 1024))]
    pub title: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_title: Option<String>,
    #[garde(skip)]
    pub artist_id: Option<Uuid>,
    #[garde(skip)]
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArtistParams {
    #[garde(length(min = 1, max = 1024))]
    pub name: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateArtistParams {
    #[garde(length(min = 1, max = 1024))]
    pub name: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_name: Option<String>,
}

/// Resolved field values produced by the side-by-side conflict picker.
/// The client merges the two entities and submits the final field values.
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MergeArtistParams {
    #[garde(skip)]
    pub source_id: Uuid,
    #[garde(length(min = 1, max = 1024))]
    pub name: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MergeAlbumParams {
    #[garde(skip)]
    pub source_id: Uuid,
    #[garde(length(min = 1, max = 1024))]
    pub title: String,
    #[garde(length(min = 0, max = 1024))]
    pub sort_title: String,
    #[garde(skip)]
    pub artist_id: Option<Uuid>,
    #[garde(skip)]
    pub date: Option<NaiveDate>,
    /// `None` keeps the target's existing cover, `Some("")`/empty would be
    /// nonsensical, so omit to leave unchanged; otherwise the filename to set
    /// (must already exist on disk — typically the target's or source's
    /// existing `cover_path`).
    #[garde(skip)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardDeleteQuery {
    #[serde(default)]
    pub hard: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_search_limit")]
    pub limit: i32,
    #[serde(default)]
    pub artist_id: Option<Uuid>,
}

fn default_search_limit() -> i32 {
    20
}
