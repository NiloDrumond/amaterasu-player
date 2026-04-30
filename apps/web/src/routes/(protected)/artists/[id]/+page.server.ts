import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getArtist, getArtistAlbums } from '$lib/services/artist-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const [artistResult, albumsResult] = await Promise.all([
		getArtist(fetch, params.id),
		getArtistAlbums(fetch, params.id),
	]);

	if (artistResult.status === 404) {
		error(404, 'Artist not found');
	}

	if (artistResult.error || albumsResult.error) {
		error(500, 'Failed to load artist');
	}

	return {
		artist: artistResult.data,
		albums: albumsResult.data,
	};
};
