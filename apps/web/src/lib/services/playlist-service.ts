import type { GetPlaylistsResponse } from '$lib/bindings/response/playlist/get-playlists-response';
import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
import type { RecentPlaylistResponse } from '$lib/bindings/response/playlist/recent-playlist-response';
import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
import type { CreatePlaylistParams } from '$lib/bindings/request/playlist/create-playlist-params';
import type { RenamePlaylistParams } from '$lib/bindings/request/playlist/rename-playlist-params';
import type { AddTracksParams } from '$lib/bindings/request/playlist/add-tracks-params';
import type { ReorderTrackParams } from '$lib/bindings/request/playlist/reorder-track-params';
import type { UpdatePlaylistFilterParams } from '$lib/bindings/request/playlist/update-playlist-filter-params';
import type { PaginationParams } from '$lib/bindings/request/common/pagination-params';
import type { SortDir } from '$lib/bindings/request/common/sort-dir';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export interface GetPlaylistsParams extends PaginationParams {
	q?: string | null;
	sort?: string | null;
	dir?: SortDir | null;
}

export function getPlaylists(
	fetch: Fetch,
	params?: GetPlaylistsParams,
): Promise<Result<GetPlaylistsResponse>> {
	if (!params) return api<GetPlaylistsResponse>(fetch, '/api/playlists');
	const search = new URLSearchParams();
	search.set('limit', String(params.limit));
	search.set('offset', String(params.offset));
	if (params.q) search.set('q', params.q);
	if (params.sort) {
		search.set('sort', params.sort);
		search.set('dir', params.dir ?? 'asc');
	}
	return api<GetPlaylistsResponse>(fetch, `/api/playlists?${search.toString()}`);
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

export function updatePlaylistFilter(
	fetch: Fetch,
	id: string,
	params: UpdatePlaylistFilterParams,
): Promise<Result<PlaylistResponse>> {
	return api<PlaylistResponse>(fetch, `/api/playlists/${id}/filter`, {
		method: 'PATCH',
		body: params,
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

export function getRecentPlaylists(
	fetch: Fetch,
	limit = 10,
): Promise<Result<RecentPlaylistResponse[]>> {
	return api<RecentPlaylistResponse[]>(fetch, `/api/playlists/recent?limit=${limit}`);
}
