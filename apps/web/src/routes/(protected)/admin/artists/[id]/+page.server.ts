import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getAdminArtist } from '$lib/services/admin-service';
import { getArtistAlbums } from '$lib/services/artist-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const { data: artist, error: err } = await getAdminArtist(fetch, params.id);
	if (err || !artist) error(404, err ?? 'Artist not found');

	const { data: albums } = await getArtistAlbums(fetch, artist.id);
	return { artist, albums: albums ?? [] };
};
