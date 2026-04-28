import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import { getContext, setContext } from 'svelte';

const PLAYER_KEY = Symbol.for('amaterasu-player');

export class PlayerState {
	queue = $state<TrackResponse[]>([]);
	index = $state(0);
	isPlaying = $state(false);
	currentTime = $state(0);
	duration = $state(0);
	volume = $state(1);
	queueOpen = $state(false);

	currentTrack = $derived<TrackResponse | null>(this.queue[this.index] ?? null);
	streamUrl = $derived<string | null>(
		this.currentTrack ? `/api/tracks/${this.currentTrack.id}/stream` : null,
	);
	hasNext = $derived(this.index < this.queue.length - 1);
	hasPrev = $derived(this.index > 0);

	playQueue(tracks: TrackResponse[], startIndex: number) {
		if (tracks.length === 0) return;
		this.queue = tracks;
		this.index = Math.max(0, Math.min(startIndex, tracks.length - 1));
		this.currentTime = 0;
		this.isPlaying = true;
	}

	toggle() {
		if (!this.currentTrack) return;
		this.isPlaying = !this.isPlaying;
	}

	next() {
		if (this.hasNext) {
			this.index += 1;
			this.currentTime = 0;
			this.isPlaying = true;
		} else {
			this.isPlaying = false;
		}
	}

	prev() {
		if (this.hasPrev) {
			this.index -= 1;
			this.currentTime = 0;
			this.isPlaying = true;
		}
	}

	onEnded() {
		this.next();
	}

	toggleQueue() {
		this.queueOpen = !this.queueOpen;
	}

	jumpTo(i: number) {
		if (i < 0 || i >= this.queue.length) return;
		this.index = i;
		this.currentTime = 0;
		this.isPlaying = true;
	}

	removeAt(i: number) {
		if (i < 0 || i >= this.queue.length) return;
		const wasCurrent = i === this.index;
		this.queue = [...this.queue.slice(0, i), ...this.queue.slice(i + 1)];
		if (i < this.index) this.index -= 1;
		if (wasCurrent) {
			if (this.queue.length === 0) {
				this.isPlaying = false;
				this.index = 0;
			} else if (this.index >= this.queue.length) {
				this.index = this.queue.length - 1;
				this.isPlaying = false;
			}
			this.currentTime = 0;
		}
	}

	reorder(from: number, to: number) {
		if (from === to || from < 0 || from >= this.queue.length) return;
		const currentId = this.currentTrack?.id;
		const next = [...this.queue];
		const [item] = next.splice(from, 1);
		const insertAt = Math.max(0, Math.min(to, next.length));
		next.splice(insertAt, 0, item);
		this.queue = next;
		if (currentId) {
			const found = next.findIndex((t) => t.id === currentId);
			if (found !== -1) this.index = found;
		}
	}

	playNext(tracks: TrackResponse[]) {
		if (tracks.length === 0) return;
		const insertAt = this.index + 1;
		this.queue = [...this.queue.slice(0, insertAt), ...tracks, ...this.queue.slice(insertAt)];
	}

	playLater(tracks: TrackResponse[]) {
		if (tracks.length === 0) return;
		this.queue = [...this.queue, ...tracks];
	}
}

export function setPlayer(): PlayerState {
	return setContext(PLAYER_KEY, new PlayerState());
}

export function getPlayer(): PlayerState {
	return getContext(PLAYER_KEY);
}
