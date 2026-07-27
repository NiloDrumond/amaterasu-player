import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
import type { TrackResponse } from '$lib/bindings/response/track/track-response';

/**
 * Widens a playlist track into a plain track for the player and the tracks
 * table.
 *
 * `PlaylistTrackResponse` is a `TrackResponse` plus the membership fields, so
 * this only has to drop those three. The return type is what enforces that: if
 * the API ever stops sending one of the track fields, this stops compiling
 * rather than handing the player an object with holes in it. Do not replace it
 * with a cast.
 */
export function asTrackResponse({
	playlistTrackId: _playlistTrackId,
	position: _position,
	addedAt: _addedAt,
	...track
}: PlaylistTrackResponse): TrackResponse {
	return track;
}
