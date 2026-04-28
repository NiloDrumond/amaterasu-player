import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions: Actions = {
	default: async ({ request, fetch, url }) => {
		const data = await request.formData();
		const email = data.get('email');
		const password = data.get('password');

		const res = await fetch('/api/auth/sign-in', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ email, password }),
		});

		if (!res.ok) {
			const body = (await res.json()) as { error: string };
			return fail(400, { error: body.error });
		}

		let redirectTo = url.searchParams.get('redirectTo') ?? '/';
		if (!redirectTo.startsWith('/') || redirectTo.startsWith('//')) {
			redirectTo = '/';
		}
		redirect(303, redirectTo);
	},
};
