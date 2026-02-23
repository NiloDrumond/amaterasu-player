use ffmpeg_next::format::context::Input;

use crate::db::entities::Artist;

pub struct ScannedArtistMetadata {
    pub name: Option<String>,
    pub sort_name: Option<String>,
}

impl From<ScannedArtistMetadata> for Artist {
    fn from(value: ScannedArtistMetadata) -> Self {
        let name = value.name.unwrap_or("unknown".to_string());
        Artist {
            name: name.clone(),
            sort_name: value.sort_name.unwrap_or(name),
            ..Default::default()
        }
    }
}

impl ScannedArtistMetadata {
    pub fn from_track_context(ictx: &Input, folder_name: Option<String>) -> Self {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> { metadata.get(key).map(|v| v.to_string()) };

        Self {
            name: get_string("artist")
                .or_else(|| get_string("artist_sort"))
                .or(folder_name),
            sort_name: get_string("artistsort").or_else(|| get_string("sort_artist")),
        }
    }

    pub fn from_album_context(ictx: &Input, folder_name: Option<String>) -> Self {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> { metadata.get(key).map(|v| v.to_string()) };

        Self {
            name: get_string("album_artist")
                .or_else(|| get_string("albumartist"))
                .or(folder_name),
            sort_name: get_string("albumartistsort").or_else(|| get_string("sort_album_artist")),
        }
    }
}
