import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { signIn } from '$lib/services/auth-service';

export const actions: Actions = {
	default: async ({ request, fetch, url }) => {
		const data = await request.formData();
		const email = data.get('email') as string;
		const password = data.get('password') as string;

		const { error } = await signIn(fetch, { email, password });

		if (error) {
			return fail(400, { error });
		}

		let redirectTo = url.searchParams.get('redirectTo') ?? '/';
		if (!redirectTo.startsWith('/') || redirectTo.startsWith('//')) {
			redirectTo = '/';
		}
		redirect(303, redirectTo);
	},
};
