import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getAdminAlbum, getAdminArtist } from '$lib/services/admin-service';
import { getAlbumTracks } from '$lib/services/album-service';

export const load: PageServerLoad = async ({ fetch, params }) => {
	const { data: album, error: err } = await getAdminAlbum(fetch, params.id);
	if (err || !album) error(404, err ?? 'Album not found');

	const [artist, tracksRes] = await Promise.all([
		album.artistId
			? getAdminArtist(fetch, album.artistId).then((r) => r.data)
			: Promise.resolve(null),
		getAlbumTracks(fetch, album.id),
	]);

	return { album, artist, tracks: tracksRes.data ?? [] };
};
