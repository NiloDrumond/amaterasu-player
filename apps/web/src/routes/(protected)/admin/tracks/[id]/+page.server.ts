import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getAdminTrack, getAdminAlbum, getAdminArtist } from '$lib/services/admin-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const { data: track, error: trackErr } = await getAdminTrack(fetch, params.id);
	if (trackErr || !track) error(404, trackErr ?? 'Track not found');

	const [album, artist] = await Promise.all([
		track.albumId ? getAdminAlbum(fetch, track.albumId).then((r) => r.data) : Promise.resolve(null),
		track.artistId
			? getAdminArtist(fetch, track.artistId).then((r) => r.data)
			: Promise.resolve(null),
	]);

	return { track, album, artist };
};
