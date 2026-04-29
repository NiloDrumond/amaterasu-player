import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getPlaylist, getPlaylistTracks } from '$lib/services/playlist-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const [playlistResult, tracksResult] = await Promise.all([
		getPlaylist(fetch, params.id),
		getPlaylistTracks(fetch, params.id),
	]);

	if (playlistResult.status === 404) {
		error(404, 'Playlist not found');
	}

	if (playlistResult.error || tracksResult.error) {
		error(500, 'Failed to load playlist');
	}

	return {
		playlist: playlistResult.data!,
		tracks: tracksResult.data!,
	};
};
