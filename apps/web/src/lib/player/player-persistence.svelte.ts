import type { PlayerSnapshot, PlayerState } from './player.svelte';

const STORAGE_KEY = 'amaterasu-player-state';
const SCHEMA_VERSION = 1;
const SAVE_DEBOUNCE_MS = 500;

interface PersistedPlayerState extends PlayerSnapshot {
	version: typeof SCHEMA_VERSION;
}

function readFromStorage(): PlayerSnapshot | null {
	if (typeof window === 'undefined') return null;
	try {
		const raw = window.localStorage.getItem(STORAGE_KEY);
		if (!raw) return null;
		const parsed = JSON.parse(raw) as PersistedPlayerState;
		if (parsed.version !== SCHEMA_VERSION) return null;
		if (!Array.isArray(parsed.queue) || parsed.queue.length === 0) return null;
		return {
			queue: parsed.queue,
			originalQueue: parsed.originalQueue ?? null,
			index: typeof parsed.index === 'number' ? parsed.index : 0,
			shuffleEnabled: !!parsed.shuffleEnabled,
			repeatMode: parsed.repeatMode ?? 'off',
		};
	} catch (err) {
		console.warn('[player-persistence] failed to read', err);
		return null;
	}
}

function writeToStorage(snapshot: PlayerSnapshot) {
	if (typeof window === 'undefined') return;
	try {
		if (snapshot.queue.length === 0) {
			window.localStorage.removeItem(STORAGE_KEY);
			return;
		}
		const payload: PersistedPlayerState = { version: SCHEMA_VERSION, ...snapshot };
		window.localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
	} catch (err) {
		console.warn('[player-persistence] failed to write', err);
	}
}

export function hydratePlayerFromStorage(player: PlayerState) {
	const restored = readFromStorage();
	if (restored) player.restore(restored);
}

export function setupPlayerPersistence(player: PlayerState) {
	if (typeof window === 'undefined') return;
	let timer: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		const snapshot: PlayerSnapshot = player.snapshot();

		if (timer !== null) clearTimeout(timer);
		timer = setTimeout(() => {
			timer = null;
			writeToStorage(snapshot);
		}, SAVE_DEBOUNCE_MS);

		return () => {
			if (timer !== null) {
				clearTimeout(timer);
				timer = null;
			}
		};
	});

	$effect(() => {
		const handler = () => {
			if (timer !== null) {
				clearTimeout(timer);
				timer = null;
			}
			writeToStorage(player.snapshot());
		};
		window.addEventListener('beforeunload', handler);
		return () => window.removeEventListener('beforeunload', handler);
	});
}

export const __internal = { STORAGE_KEY, SCHEMA_VERSION, readFromStorage, writeToStorage };
