import type { PinPlaylistParams } from '$lib/bindings/request/me/pin-playlist-params';
import type { ReorderPinnedPlaylistsParams } from '$lib/bindings/request/me/reorder-pinned-playlists-params';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function pinPlaylist(fetch: Fetch, playlistId: string): Promise<Result<void>> {
	const body: PinPlaylistParams = { playlistId };
	return api<void>(fetch, '/api/me/pinned-playlists', { method: 'POST', body });
}

export function unpinPlaylist(fetch: Fetch, playlistId: string): Promise<Result<void>> {
	return api<void>(fetch, `/api/me/pinned-playlists/${playlistId}`, { method: 'DELETE' });
}

export function reorderPinnedPlaylists(
	fetch: Fetch,
	orderedPlaylistIds: string[],
): Promise<Result<void>> {
	const body: ReorderPinnedPlaylistsParams = { orderedPlaylistIds };
	return api<void>(fetch, '/api/me/pinned-playlists/reorder', { method: 'PATCH', body });
}
