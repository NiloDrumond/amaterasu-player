import type { UserPreferences } from '$lib/bindings/response/user/user-preferences';
import type { VisibilityState } from '@tanstack/table-core';
import { updatePreferences } from '$lib/services/auth-service';

const SAVE_DEBOUNCE_MS = 400;

let preferences = $state<UserPreferences>({ tableColumns: {} });
let saveTimer: ReturnType<typeof setTimeout> | null = null;

export function initUserPreferences(initial: UserPreferences) {
	preferences = {
		tableColumns: { ...(initial.tableColumns ?? {}) },
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

function schedulePersist() {
	if (typeof window === 'undefined') return;
	if (saveTimer !== null) clearTimeout(saveTimer);
	saveTimer = setTimeout(() => {
		saveTimer = null;
		const snapshot: UserPreferences = {
			tableColumns: { ...preferences.tableColumns },
		};
		void updatePreferences(fetch, snapshot);
	}, SAVE_DEBOUNCE_MS);
}
