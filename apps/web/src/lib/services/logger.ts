import { browser } from '$app/environment';

// Buffered client-side log/error reporter. Errors and manual log calls are
// queued and flushed in batches to `/api/client-logs`, which forwards them to
// Loki as the `service=amaterasu-web` stream. The user identity is stamped
// server-side from the session cookie, so nothing identifying is sent here.

export type LogLevel = 'error' | 'warn' | 'info';

type LogContext = Record<string, unknown>;

interface QueuedEntry {
	level: LogLevel;
	message: string;
	stack?: string;
	url: string;
	route?: string;
	ts: number;
	context?: LogContext;
}

const ENDPOINT = '/api/client-logs';
const FLUSH_INTERVAL_MS = 5000;
const MAX_QUEUE = 20;
const MAX_TEXT_LEN = 8192;

let queue: QueuedEntry[] = [];
let flushTimer: ReturnType<typeof setTimeout> | null = null;
// True while a flush request is in flight. Prevents overlapping sends and,
// together with raw `fetch` + swallowed errors, prevents any feedback loop
// where reporting a failure would itself generate more logs.
let sending = false;
let initialized = false;

function enqueue(level: LogLevel, message: string, context?: LogContext, stack?: string) {
	if (!browser) return;
	try {
		const route = typeof context?.route === 'string' ? context.route : undefined;
		queue.push({
			level,
			message: truncate(message),
			stack: stack ? truncate(stack) : undefined,
			url: location.href,
			route,
			ts: Date.now(),
			context: stripContext(context),
		});
	} catch {
		// Collecting context must never throw into application code.
		return;
	}

	if (queue.length >= MAX_QUEUE) {
		flush();
	} else {
		scheduleFlush();
	}
}

function scheduleFlush() {
	if (flushTimer != null) return;
	flushTimer = setTimeout(flush, FLUSH_INTERVAL_MS);
}

function flush() {
	if (flushTimer != null) {
		clearTimeout(flushTimer);
		flushTimer = null;
	}
	if (!browser || sending || queue.length === 0) return;

	const batch = queue;
	queue = [];
	sending = true;

	// Raw `fetch` (not the `api()` wrapper) so a failure here can't be reported
	// back through the logger. `keepalive` lets it survive a navigation.
	fetch(ENDPOINT, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ entries: batch }),
		keepalive: true,
	})
		.catch(() => {
			// Drop on failure; never re-enqueue.
		})
		.finally(() => {
			sending = false;
			if (queue.length > 0) scheduleFlush();
		});
}

function flushBeacon() {
	if (!browser || queue.length === 0) return;
	const batch = queue;
	queue = [];
	try {
		const blob = new Blob([JSON.stringify({ entries: batch })], { type: 'application/json' });
		navigator.sendBeacon(ENDPOINT, blob);
	} catch {
		// sendBeacon can throw on oversized payloads; drop quietly.
	}
}

function truncate(s: string): string {
	return s.length > MAX_TEXT_LEN ? `${s.slice(0, MAX_TEXT_LEN)}…` : s;
}

// `route` is promoted to a top-level field, so drop it from the merged context.
function stripContext(context?: LogContext): LogContext | undefined {
	if (!context) return undefined;
	const { route: _route, ...rest } = context;
	return Object.keys(rest).length > 0 ? rest : undefined;
}

function describe(err: unknown): { message: string; stack?: string } {
	if (err instanceof Error) {
		return { message: err.message || err.name, stack: err.stack };
	}
	if (typeof err === 'string') return { message: err };
	try {
		return { message: JSON.stringify(err) };
	} catch {
		return { message: String(err) };
	}
}

export const logger = {
	error(message: string, context?: LogContext) {
		enqueue('error', message, context);
	},
	warn(message: string, context?: LogContext) {
		enqueue('warn', message, context);
	},
	info(message: string, context?: LogContext) {
		enqueue('info', message, context);
	},
	/** Capture an arbitrary thrown value, extracting its message and stack. */
	captureError(err: unknown, context?: LogContext) {
		const { message, stack } = describe(err);
		enqueue('error', message, context, stack);
	},
};

/**
 * Registers global error listeners and flush-on-unload handlers. Call once,
 * from `hooks.client.ts`. Safe to call repeatedly (no-ops after the first).
 */
export function initClientLogging() {
	if (!browser || initialized) return;
	initialized = true;

	window.addEventListener('error', (event) => {
		logger.captureError(event.error ?? event.message, {
			source: 'window.onerror',
			filename: event.filename,
			line: event.lineno,
			column: event.colno,
		});
	});

	window.addEventListener('unhandledrejection', (event) => {
		logger.captureError(event.reason, { source: 'unhandledrejection' });
	});

	document.addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'hidden') flushBeacon();
	});
	window.addEventListener('pagehide', flushBeacon);
}
