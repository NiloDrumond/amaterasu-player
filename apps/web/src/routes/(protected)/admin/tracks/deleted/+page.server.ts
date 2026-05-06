import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { listDeletedTracks } from '$lib/services/admin-service';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data, error: err } = await listDeletedTracks(fetch);
	if (err || !data) error(500, err ?? 'Failed to load');
	return { tracks: data };
};
