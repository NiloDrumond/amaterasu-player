import type { HandleServerError } from '@sveltejs/kit';

// SSR errors run in the SvelteKit (Bun) server, whose stdout is not shipped to
// Loki. Forward them to the backend ingest endpoint so they land in the same
// `service=amaterasu-web` stream as browser errors. `event.fetch` resolves
// `/api/*` against the incoming origin (through nginx → backend) and carries
// the session cookie, exactly like the load-function API calls do.
export const handleError: HandleServerError = async ({ error, event, status, message }) => {
	if (status !== 404) {
		try {
			await event.fetch('/api/client-logs', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					entries: [
						{
							level: 'error',
							message: error instanceof Error ? error.message : String(message),
							stack: error instanceof Error ? error.stack : undefined,
							url: event.url.href,
							route: event.route?.id ?? undefined,
							ts: Date.now(),
							context: { source: 'ssr', status },
						},
					],
				}),
			});
		} catch {
			// Never let error reporting mask or replace the original error.
		}
	}
	return { message };
};
