import { error } from '@sveltejs/kit';
import type { ArtistResponse } from '$lib/bindings/response/artist/artist-response';
import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const [artistRes, albumsRes] = await Promise.all([
		fetch(`/api/artists/${params.id}`),
		fetch(`/api/artists/${params.id}/albums`),
	]);

	if (artistRes.status === 404) {
		error(404, 'Artist not found');
	}

	if (!artistRes.ok || !albumsRes.ok) {
		error(500, 'Failed to load artist');
	}

	return {
		artist: (await artistRes.json()) as ArtistResponse,
		albums: (await albumsRes.json()) as AlbumResponse[],
	};
};
