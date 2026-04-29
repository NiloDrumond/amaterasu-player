import type { PageServerLoad } from './$types';
import { getPlaylists } from '$lib/services/playlist-service';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data: playlists, error } = await getPlaylists(fetch);

	if (error) {
		return { playlists: null as any, error: { status: 500, message: error } };
	}

	return { playlists: playlists!, error: null };
};
