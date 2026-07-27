import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';
import type { HomeItemResponse } from '$lib/bindings/response/me/home-item-response';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getPinnedPlaylists(fetch: Fetch): Promise<Result<PlaylistResponse[]>> {
	return api<PlaylistResponse[]>(fetch, '/api/me/pinned-playlists');
}

export function getListenAgain(fetch: Fetch, limit = 20): Promise<Result<HomeItemResponse[]>> {
	return api<HomeItemResponse[]>(fetch, `/api/me/recommendations/listen-again?limit=${limit}`);
}

export function getForgottenFavorites(
	fetch: Fetch,
	limit = 20,
): Promise<Result<HomeItemResponse[]>> {
	return api<HomeItemResponse[]>(
		fetch,
		`/api/me/recommendations/forgotten-favorites?limit=${limit}`,
	);
}

export async function getRecentlyAddedAlbums(
	fetch: Fetch,
	limit = 20,
): Promise<Result<AlbumResponse[]>> {
	const search = new URLSearchParams({
		limit: String(limit),
		offset: '0',
		sort: 'recent',
		dir: 'desc',
	});
	const result = await api<GetAlbumsResponse>(fetch, `/api/albums?${search.toString()}`);
	if (result.error !== null) {
		return result;
	}
	return { data: result.data.data, error: null, status: result.status };
}
