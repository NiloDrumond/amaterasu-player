import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getReviewQueue } from '$lib/services/admin-service';

const PAGE_SIZE = 20;

export const load: PageServerLoad = async ({ fetch, url }) => {
	const offset = Math.max(0, Number(url.searchParams.get('offset') ?? '0') || 0);
	const { data, error: err } = await getReviewQueue(fetch, { offset, limit: PAGE_SIZE });
	if (err || !data) error(500, err ?? 'Failed to load review queue');

	return { queue: data, offset, pageSize: PAGE_SIZE };
};
