import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getAlbum, getAlbumTracks } from '$lib/services/album-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const [albumResult, tracksResult] = await Promise.all([
		getAlbum(fetch, params.id),
		getAlbumTracks(fetch, params.id),
	]);

	if (albumResult.status === 404) {
		error(404, 'Album not found');
	}

	if (albumResult.error || tracksResult.error) {
		error(500, 'Failed to load album');
	}

	return {
		album: albumResult.data,
		tracks: tracksResult.data,
	};
};
