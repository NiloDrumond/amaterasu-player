import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import { scrobbleTrack } from '$lib/services/track-service';
import { getContext, setContext } from 'svelte';

const PLAYER_KEY = Symbol.for('amaterasu-player');

export interface PlaybackContext {
	albumId?: string | null;
	playlistId?: string | null;
}

// Scrobble threshold: >50% of track duration OR >4 minutes, whichever comes first.
const SCROBBLE_MIN_FRACTION = 0.5;
const SCROBBLE_MIN_SECONDS = 240;

export class PlayerState {
	queue = $state<TrackResponse[]>([]);
	index = $state(0);
	isPlaying = $state(false);
	currentTime = $state(0);
	duration = $state(0);
	volume = $state(1);
	previousVolume = $state(1);
	queueOpen = $state(false);
	context = $state<PlaybackContext>({});

	private scrobbledTrackKey: string | null = null;

	currentTrack = $derived<TrackResponse | null>(this.queue[this.index] ?? null);
	streamUrl = $derived<string | null>(
		this.currentTrack ? `/api/tracks/${this.currentTrack.id}/stream` : null,
	);
	hasNext = $derived(this.index < this.queue.length - 1);
	hasPrev = $derived(this.index > 0);

	playQueue(tracks: TrackResponse[], startIndex: number, context: PlaybackContext = {}) {
		if (tracks.length === 0) return;
		this.queue = tracks;
		this.index = Math.max(0, Math.min(startIndex, tracks.length - 1));
		this.currentTime = 0;
		this.isPlaying = true;
		this.context = context;
		this.resetScrobble();
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
			this.resetScrobble();
		} else {
			this.isPlaying = false;
		}
	}

	prev() {
		if (this.hasPrev) {
			this.index -= 1;
			this.currentTime = 0;
			this.isPlaying = true;
			this.resetScrobble();
		}
	}

	onEnded() {
		this.next();
	}

	toggleMute() {
		if (this.volume > 0) {
			this.previousVolume = this.volume;
			this.volume = 0;
		} else {
			this.volume = this.previousVolume || 1;
		}
	}

	toggleQueue() {
		this.queueOpen = !this.queueOpen;
	}

	jumpTo(i: number) {
		if (i < 0 || i >= this.queue.length) return;
		this.index = i;
		this.currentTime = 0;
		this.isPlaying = true;
		this.resetScrobble();
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
			this.resetScrobble();
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

	/**
	 * Called on each timeupdate while playing. Scrobbles once per track play when
	 * the listener has crossed >50% of duration or >4 minutes.
	 */
	maybeScrobble() {
		const track = this.currentTrack;
		if (!track) return;
		const playKey = `${track.id}:${this.index}`;
		if (this.scrobbledTrackKey === playKey) return;

		const durationSec = track.durationMs / 1000;
		const halfway = durationSec > 0 && this.currentTime >= durationSec * SCROBBLE_MIN_FRACTION;
		const longEnough = this.currentTime >= SCROBBLE_MIN_SECONDS;
		if (!halfway && !longEnough) return;

		this.scrobbledTrackKey = playKey;
		void scrobbleTrack(fetch, track.id, {
			contextAlbumId: this.context.albumId ?? null,
			contextPlaylistId: this.context.playlistId ?? null,
		});
	}

	private resetScrobble() {
		this.scrobbledTrackKey = null;
	}
}

export function setPlayer(): PlayerState {
	return setContext(PLAYER_KEY, new PlayerState());
}

export function getPlayer(): PlayerState {
	return getContext(PLAYER_KEY);
}
