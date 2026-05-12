use ffmpeg_next::format::context::Input;

pub struct ScannedArtistMetadata {
    pub name: Option<String>,
    pub sort_name: Option<String>,
}

impl ScannedArtistMetadata {
    pub fn from_track_context(ictx: &Input, folder_name: Option<String>) -> Self {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> {
            metadata
                .get(key)
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
        };

        Self {
            name: get_string("artist")
                .or_else(|| get_string("artist_sort"))
                .or(folder_name),
            sort_name: get_string("artistsort").or_else(|| get_string("sort_artist")),
        }
    }

    pub fn from_album_context(ictx: &Input, folder_name: Option<String>) -> Self {
        let metadata = ictx.metadata();

        let get_string = |key: &str| -> Option<String> {
            metadata
                .get(key)
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
        };

        Self {
            name: get_string("album_artist")
                .or_else(|| get_string("albumartist"))
                .or(folder_name),
            sort_name: get_string("albumartistsort").or_else(|| get_string("sort_album_artist")),
        }
    }
}
