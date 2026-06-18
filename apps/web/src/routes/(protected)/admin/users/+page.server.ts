import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getAdminUsers } from '$lib/services/admin-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: users, error: errorMessage } = await getAdminUsers(fetch, { limit, offset });

	if (errorMessage) {
		return { users: null, error: { status: 500, message: errorMessage }, page };
	}
	if (!users) {
		error(500, 'Failed to load users');
	}

	return { users, error: null, page };
};
