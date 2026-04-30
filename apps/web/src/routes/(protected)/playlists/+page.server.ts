import type { PageServerLoad } from './$types';
import { getPlaylists } from '$lib/services/playlist-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data: playlists, error: errorMessage } = await getPlaylists(fetch);

	if (errorMessage) {
		return { playlists: null, error: { status: 500, message: errorMessage } };
	}
	if (!playlists) {
		error(500, 'Failed to load playlists');
	}

	return { playlists: playlists, error: null };
};
