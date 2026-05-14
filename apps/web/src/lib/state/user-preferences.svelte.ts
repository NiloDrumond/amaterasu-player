import type { UserPreferences } from '$lib/bindings/response/user/user-preferences';
import type { VisibilityState } from '@tanstack/table-core';
import { updatePreferences } from '$lib/services/auth-service';

const SAVE_DEBOUNCE_MS = 400;

let preferences = $state<UserPreferences>({ tableColumns: {}, volume: null });
let saveTimer: ReturnType<typeof setTimeout> | null = null;

export function initUserPreferences(initial: UserPreferences) {
	preferences = {
		tableColumns: { ...(initial.tableColumns ?? {}) },
		volume: initial.volume ?? null,
	};
}

export function getColumnVisibility(storageKey: string): VisibilityState {
	return preferences.tableColumns[storageKey] ?? {};
}

export function setColumnVisibility(storageKey: string, value: VisibilityState) {
	preferences.tableColumns = {
		...preferences.tableColumns,
		[storageKey]: { ...value },
	};
	schedulePersist();
}

export function getVolume(): number | null {
	return preferences.volume;
}

export function setVolume(value: number) {
	preferences.volume = value;
	schedulePersist();
}

function schedulePersist() {
	if (typeof window === 'undefined') return;
	if (saveTimer !== null) clearTimeout(saveTimer);
	saveTimer = setTimeout(() => {
		saveTimer = null;
		const snapshot: UserPreferences = {
			tableColumns: { ...preferences.tableColumns },
			volume: preferences.volume,
		};
		void updatePreferences(fetch, snapshot);
	}, SAVE_DEBOUNCE_MS);
}
