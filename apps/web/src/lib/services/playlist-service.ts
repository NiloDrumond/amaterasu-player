import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
import type { CreatePlaylistParams } from '$lib/bindings/request/playlist/create-playlist-params';
import type { RenamePlaylistParams } from '$lib/bindings/request/playlist/rename-playlist-params';
import type { AddTracksParams } from '$lib/bindings/request/playlist/add-tracks-params';
import type { ReorderTrackParams } from '$lib/bindings/request/playlist/reorder-track-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getPlaylists(fetch: Fetch): Promise<Result<PlaylistResponse[]>> {
	return api<PlaylistResponse[]>(fetch, '/api/playlists');
}

export function getPlaylist(fetch: Fetch, id: string): Promise<Result<PlaylistResponse>> {
	return api<PlaylistResponse>(fetch, `/api/playlists/${id}`);
}

export function getPlaylistTracks(
	fetch: Fetch,
	id: string,
): Promise<Result<PlaylistTrackResponse[]>> {
	return api<PlaylistTrackResponse[]>(fetch, `/api/playlists/${id}/tracks`);
}

export function createPlaylist(
	fetch: Fetch,
	params: CreatePlaylistParams,
): Promise<Result<PlaylistResponse>> {
	return api<PlaylistResponse>(fetch, '/api/playlists', { method: 'POST', body: params });
}

export function renamePlaylist(
	fetch: Fetch,
	id: string,
	params: RenamePlaylistParams,
): Promise<Result<PlaylistResponse>> {
	return api<PlaylistResponse>(fetch, `/api/playlists/${id}`, { method: 'PATCH', body: params });
}

export function deletePlaylist(fetch: Fetch, id: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/playlists/${id}`, { method: 'DELETE' });
}

export function addTracksToPlaylist(
	fetch: Fetch,
	id: string,
	params: AddTracksParams,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/playlists/${id}/tracks`, { method: 'POST', body: params });
}

export function removeTrackFromPlaylist(
	fetch: Fetch,
	playlistId: string,
	playlistTrackId: string,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/playlists/${playlistId}/tracks/${playlistTrackId}`, {
		method: 'DELETE',
	});
}

export function reorderPlaylistTrack(
	fetch: Fetch,
	playlistId: string,
	trackId: string,
	params: ReorderTrackParams,
): Promise<Result<void>> {
	return api<void>(fetch, `/api/playlists/${playlistId}/tracks/${trackId}`, {
		method: 'PATCH',
		body: params,
	});
}
