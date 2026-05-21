import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { __internal } from './player-persistence.svelte';
import type { PlayerSnapshot, QueueItem } from './player.svelte';

const { STORAGE_KEY, readFromStorage, writeToStorage } = __internal;

function makeFakeStorage(): Storage {
	let store: Record<string, string> = {};
	return {
		getItem: (k: string) => (k in store ? store[k] : null),
		setItem: (k: string, v: string) => {
			store[k] = String(v);
		},
		removeItem: (k: string) => {
			delete store[k];
		},
		clear: () => {
			store = {};
		},
		key: (i: number) => Object.keys(store)[i] ?? null,
		get length() {
			return Object.keys(store).length;
		},
	};
}

const item = (id: string): QueueItem => ({
	track: {
		id,
		title: `t-${id}`,
		artist: null,
		album: null,
		durationMs: 1000,
	} as QueueItem['track'],
	origin: {},
});

const baseSnapshot: PlayerSnapshot = {
	queue: [item('a'), item('b')],
	originalQueue: null,
	index: 1,
	shuffleEnabled: false,
	repeatMode: 'all',
};

describe('player-persistence', () => {
	let storage: Storage;

	beforeEach(() => {
		storage = makeFakeStorage();
		vi.stubGlobal('window', { localStorage: storage });
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('writes and reads a snapshot round-trip', () => {
		writeToStorage(baseSnapshot);
		expect(readFromStorage()).toEqual(baseSnapshot);
	});

	it('removes the entry when queue is empty', () => {
		storage.setItem(STORAGE_KEY, 'sentinel');
		writeToStorage({ ...baseSnapshot, queue: [] });
		expect(storage.getItem(STORAGE_KEY)).toBeNull();
	});

	it('returns null when nothing is stored', () => {
		expect(readFromStorage()).toBeNull();
	});

	it('returns null on invalid JSON', () => {
		storage.setItem(STORAGE_KEY, '{not-json');
		expect(readFromStorage()).toBeNull();
	});

	it('returns null on version mismatch', () => {
		storage.setItem(STORAGE_KEY, JSON.stringify({ ...baseSnapshot, version: 999 }));
		expect(readFromStorage()).toBeNull();
	});

	it('returns null when queue is empty in stored payload', () => {
		storage.setItem(STORAGE_KEY, JSON.stringify({ version: 1, queue: [] }));
		expect(readFromStorage()).toBeNull();
	});

	it('preserves originalQueue for shuffle restore', () => {
		const snap: PlayerSnapshot = {
			...baseSnapshot,
			shuffleEnabled: true,
			originalQueue: [item('a'), item('b'), item('c')],
		};
		writeToStorage(snap);
		expect(readFromStorage()).toEqual(snap);
	});
});
