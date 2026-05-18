import type { LayoutServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/services/auth-service';
import { getReviewCounts } from '$lib/services/admin-service';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
	const { data: user, error: errorMessage } = await getCurrentUser(fetch);
	if (errorMessage) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	if (!user) {
		error(500, 'Failed to load user');
	}
	if (user.role !== 'admin') {
		error(403, 'Admin only');
	}
	const { data: reviewCounts } = await getReviewCounts(fetch);
	return { user, reviewCounts };
};
