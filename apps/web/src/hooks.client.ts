import type { HandleClientError } from '@sveltejs/kit';
import { initClientLogging, logger } from '$lib/services/logger';

// Register global window error / unhandledrejection listeners and the
// flush-on-unload handlers. Runs once when the client bundle loads.
initClientLogging();

export const handleError: HandleClientError = ({ error, event, status, message }) => {
	// 404s are routine navigation noise, not actionable errors.
	if (status !== 404) {
		logger.captureError(error, {
			source: 'sveltekit-client',
			status,
			route: event.route?.id ?? undefined,
		});
	}
	return { message };
};
