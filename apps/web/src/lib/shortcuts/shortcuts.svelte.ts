/**
 * Lightweight keyboard shortcut manager.
 *
 * Single window-level listener; components register declarative bindings via
 * `useShortcut(...)` and cleanup happens automatically on unmount.
 */

export interface ShortcutDefinition {
	id: string;
	/**
	 * Key spec, case-insensitive. Examples: `'Space'`, `'f'`, `'Escape'`,
	 * `'mod+k'`, `'shift+/'`. Use `mod` for the platform meta key (Cmd on mac,
	 * Ctrl elsewhere). Multiple specs can be passed as an array.
	 */
	keys: string | string[];
	handler: (e: KeyboardEvent) => void;
	/** Optional gate evaluated at dispatch time. */
	when?: () => boolean;
	/** If true, fire even when an input-like element is focused. Default false. */
	allowInInputs?: boolean;
	/** Call preventDefault on the event when the shortcut matches. Default true. */
	preventDefault?: boolean;
	description?: string;
}

interface RegisteredShortcut extends ShortcutDefinition {
	internalId: number;
	normalized: string[];
}

const isMac =
	typeof navigator !== 'undefined' && /mac|ipad|iphone|ipod/i.test(navigator.userAgent);

function normalizeKey(spec: string): string {
	const parts = spec
		.toLowerCase()
		.split('+')
		.map((p) => p.trim())
		.filter(Boolean);

	const mods = { ctrl: false, meta: false, shift: false, alt: false };
	let key = '';
	for (const part of parts) {
		if (part === 'mod') {
			if (isMac) mods.meta = true;
			else mods.ctrl = true;
		} else if (part === 'ctrl' || part === 'control') {
			mods.ctrl = true;
		} else if (part === 'meta' || part === 'cmd' || part === 'command') {
			mods.meta = true;
		} else if (part === 'shift') {
			mods.shift = true;
		} else if (part === 'alt' || part === 'option') {
			mods.alt = true;
		} else {
			key = part === 'space' ? ' ' : part;
		}
	}
	const ordered = (['ctrl', 'meta', 'shift', 'alt'] as const).filter((m) => mods[m]);
	return [...ordered, key].join('+');
}

function eventToKey(e: KeyboardEvent): string {
	const mods: string[] = [];
	if (e.ctrlKey) mods.push('ctrl');
	if (e.metaKey) mods.push('meta');
	if (e.shiftKey) mods.push('shift');
	if (e.altKey) mods.push('alt');
	const key = e.key.length === 1 ? e.key.toLowerCase() : e.key.toLowerCase();
	return [...mods, key].join('+');
}

const INPUT_SELECTOR = 'input, textarea, select, [contenteditable=""], [contenteditable="true"]';

function isTypingTarget(): boolean {
	if (typeof document === 'undefined') return false;
	const el = document.activeElement as HTMLElement | null;
	if (!el) return false;
	return el.matches?.(INPUT_SELECTOR) ?? false;
}

class ShortcutManager {
	private shortcuts: RegisteredShortcut[] = [];
	private nextId = 1;
	private listening = false;

	register(def: ShortcutDefinition): number {
		const keys = Array.isArray(def.keys) ? def.keys : [def.keys];
		const normalized = keys.map(normalizeKey);
		const entry: RegisteredShortcut = {
			...def,
			internalId: this.nextId++,
			normalized,
		};
		this.shortcuts.push(entry);
		this.ensureListening();
		return entry.internalId;
	}

	unregister(internalId: number): void {
		this.shortcuts = this.shortcuts.filter((s) => s.internalId !== internalId);
		if (this.shortcuts.length === 0) this.stopListening();
	}

	list(): readonly RegisteredShortcut[] {
		return this.shortcuts;
	}

	private ensureListening() {
		if (this.listening || typeof window === 'undefined') return;
		window.addEventListener('keydown', this.dispatch);
		this.listening = true;
	}

	private stopListening() {
		if (!this.listening || typeof window === 'undefined') return;
		window.removeEventListener('keydown', this.dispatch);
		this.listening = false;
	}

	private dispatch = (e: KeyboardEvent) => {
		const eventKey = eventToKey(e);
		const typing = isTypingTarget();
		// Iterate from most recently registered so view-scoped shortcuts can
		// shadow globals (last registered wins).
		for (let i = this.shortcuts.length - 1; i >= 0; i--) {
			const s = this.shortcuts[i];
			if (!s.normalized.includes(eventKey)) continue;
			if (typing && !s.allowInInputs) continue;
			if (s.when && !s.when()) continue;
			if (s.preventDefault !== false) e.preventDefault();
			s.handler(e);
			return;
		}
	};
}

export const shortcutManager = new ShortcutManager();

/**
 * Register one or more shortcuts for the lifetime of the calling component.
 * Must be called from a component context (uses `$effect`).
 */
export function useShortcut(def: ShortcutDefinition | ShortcutDefinition[]): void {
	$effect(() => {
		const list = Array.isArray(def) ? def : [def];
		const ids = list.map((d) => shortcutManager.register(d));
		return () => ids.forEach((id) => shortcutManager.unregister(id));
	});
}
