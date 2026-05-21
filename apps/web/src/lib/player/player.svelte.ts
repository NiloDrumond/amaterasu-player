import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import { scrobbleTrack } from '$lib/services/track-service';
import { shuffle } from '$lib/utils/shuffle';
import { getContext, setContext } from 'svelte';

export type RepeatMode = 'off' | 'all' | 'one';

const PLAYER_KEY = Symbol.for('amaterasu-player');

export interface PlaybackContext {
	albumId?: string | null;
	playlistId?: string | null;
}

export interface QueueItem {
	track: TrackResponse;
	origin: PlaybackContext;
}

export interface PlayerSnapshot {
	queue: QueueItem[];
	originalQueue: QueueItem[] | null;
	index: number;
	shuffleEnabled: boolean;
	repeatMode: RepeatMode;
}

// Scrobble threshold: >50% of track duration OR >4 minutes, whichever comes first.
const SCROBBLE_MIN_FRACTION = 0.5;
const SCROBBLE_MIN_SECONDS = 240;

const VISUALIZER_BARS = 5;
// Frequency bins to sample for the visualizer. fftSize=64 => 32 bins; the top
// bins are usually silent, so we restrict to the low/mid range.
const VISUALIZER_BIN_START = 1;
const VISUALIZER_BIN_END = 20;
const VISUALIZER_DECAY = 0.85;

function toItems(tracks: TrackResponse[], origin: PlaybackContext): QueueItem[] {
	return tracks.map((track) => ({ track, origin }));
}

export class PlayerState {
	queue = $state<QueueItem[]>([]);
	index = $state(0);
	isPlaying = $state(false);
	currentTime = $state(0);
	duration = $state(0);
	volume = $state(1);
	previousVolume = $state(1);
	queueOpen = $state(false);
	focusedOpen = $state(false);
	shuffleEnabled = $state(false);
	repeatMode = $state<RepeatMode>('off');
	stopAfterCurrent = $state(false);
	levels = $state<number[]>(new Array(VISUALIZER_BARS).fill(0));

	private scrobbledTrackKey: string | null = null;
	private originalQueue = $state<QueueItem[] | null>(null);
	private audioCtx: AudioContext | null = null;
	private analyser: AnalyserNode | null = null;
	private freqBuffer: Uint8Array<ArrayBuffer> | null = null;
	private rafId: number | null = null;

	currentItem = $derived<QueueItem | null>(this.queue[this.index] ?? null);
	currentTrack = $derived<TrackResponse | null>(this.currentItem?.track ?? null);
	currentOrigin = $derived<PlaybackContext>(this.currentItem?.origin ?? {});
	streamUrl = $derived<string | null>(
		this.currentTrack ? `/api/tracks/${this.currentTrack.id}/stream` : null,
	);
	hasNext = $derived(
		this.index < this.queue.length - 1 || (this.repeatMode === 'all' && this.queue.length > 0),
	);
	hasPrev = $derived(this.index > 0);

	playQueue(tracks: TrackResponse[], startIndex: number, origin: PlaybackContext = {}) {
		if (tracks.length === 0) return;
		const idx = Math.max(0, Math.min(startIndex, tracks.length - 1));
		const items = toItems(tracks, origin);
		if (this.shuffleEnabled) {
			this.originalQueue = [...items];
			const before = items.slice(0, idx);
			const current = items[idx];
			const after = shuffle(items.slice(idx + 1));
			this.queue = [...before, current, ...after];
		} else {
			this.originalQueue = null;
			this.queue = items;
		}
		this.index = idx;
		this.currentTime = 0;
		this.isPlaying = true;
		this.stopAfterCurrent = false;
		this.resetScrobble();
	}

	toggle() {
		if (!this.currentTrack) return;
		this.isPlaying = !this.isPlaying;
	}

	next() {
		if (this.index < this.queue.length - 1) {
			this.index += 1;
			this.currentTime = 0;
			this.isPlaying = true;
			this.resetScrobble();
		} else if (this.repeatMode === 'all' && this.queue.length > 0) {
			this.index = 0;
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
		if (this.stopAfterCurrent) {
			this.stopAfterCurrent = false;
			this.isPlaying = false;
			this.currentTime = 0;
			return;
		}
		if (this.repeatMode === 'one') {
			this.currentTime = 0;
			this.isPlaying = true;
			this.resetScrobble();
			return;
		}
		this.next();
	}

	toggleShuffle() {
		if (!this.shuffleEnabled) {
			this.shuffleEnabled = true;
			if (this.queue.length === 0) return;
			this.originalQueue = [...this.queue];
			const before = this.queue.slice(0, this.index);
			const current = this.queue[this.index];
			const after = shuffle(this.queue.slice(this.index + 1));
			this.queue = [...before, current, ...after];
		} else {
			this.shuffleEnabled = false;
			if (this.originalQueue && this.currentTrack) {
				const currentId = this.currentTrack.id;
				const restored = this.originalQueue;
				const found = restored.findIndex((item) => item.track.id === currentId);
				this.queue = restored;
				if (found !== -1) this.index = found;
			}
			this.originalQueue = null;
		}
	}

	cycleRepeatMode() {
		this.repeatMode = this.repeatMode === 'off' ? 'all' : this.repeatMode === 'all' ? 'one' : 'off';
	}

	snapshot(): PlayerSnapshot {
		return {
			queue: this.queue,
			originalQueue: this.originalQueue,
			index: this.index,
			shuffleEnabled: this.shuffleEnabled,
			repeatMode: this.repeatMode,
		};
	}

	restore(state: PlayerSnapshot) {
		if (state.queue.length === 0) return;
		this.queue = state.queue;
		this.originalQueue = state.originalQueue;
		this.index = Math.max(0, Math.min(state.index, state.queue.length - 1));
		this.shuffleEnabled = state.shuffleEnabled;
		this.repeatMode = state.repeatMode;
		this.isPlaying = false;
		this.currentTime = 0;
		this.stopAfterCurrent = false;
		this.resetScrobble();
	}

	toggleStopAfterCurrent() {
		this.stopAfterCurrent = !this.stopAfterCurrent;
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

	openFocused() {
		this.focusedOpen = true;
	}

	closeFocused() {
		this.focusedOpen = false;
	}

	toggleFocused() {
		this.focusedOpen = !this.focusedOpen;
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
			const found = next.findIndex((it) => it.track.id === currentId);
			if (found !== -1) this.index = found;
		}
	}

	playNext(tracks: TrackResponse[], origin: PlaybackContext = {}) {
		if (tracks.length === 0) return;
		const insertAt = this.index + 1;
		const items = toItems(tracks, origin);
		this.queue = [...this.queue.slice(0, insertAt), ...items, ...this.queue.slice(insertAt)];
	}

	playLater(tracks: TrackResponse[], origin: PlaybackContext = {}) {
		if (tracks.length === 0) return;
		this.queue = [...this.queue, ...toItems(tracks, origin)];
	}

	/**
	 * Called on each timeupdate while playing. Scrobbles once per track play when
	 * the listener has crossed >50% of duration or >4 minutes.
	 */
	maybeScrobble() {
		const item = this.currentItem;
		if (!item) return;
		const playKey = `${item.track.id}:${this.index}`;
		if (this.scrobbledTrackKey === playKey) return;

		const durationSec = item.track.durationMs / 1000;
		const halfway = durationSec > 0 && this.currentTime >= durationSec * SCROBBLE_MIN_FRACTION;
		const longEnough = this.currentTime >= SCROBBLE_MIN_SECONDS;
		if (!halfway && !longEnough) return;

		this.scrobbledTrackKey = playKey;
		void scrobbleTrack(fetch, item.track.id, {
			contextAlbumId: item.origin.albumId ?? null,
			contextPlaylistId: item.origin.playlistId ?? null,
		});
	}

	private resetScrobble() {
		this.scrobbledTrackKey = null;
	}

	attachAudio(el: HTMLAudioElement) {
		if (this.audioCtx) return;
		const Ctor =
			window.AudioContext ??
			(window as unknown as { webkitAudioContext?: typeof AudioContext }).webkitAudioContext;
		if (!Ctor) return;
		this.audioCtx = new Ctor();
		const source = this.audioCtx.createMediaElementSource(el);
		const analyser = this.audioCtx.createAnalyser();
		analyser.fftSize = 64;
		analyser.smoothingTimeConstant = 0.7;
		source.connect(analyser);
		analyser.connect(this.audioCtx.destination);
		this.analyser = analyser;
		this.freqBuffer = new Uint8Array(new ArrayBuffer(analyser.frequencyBinCount));
		this.startVisualizerLoop();
	}

	resumeAudioContext() {
		if (this.audioCtx && this.audioCtx.state === 'suspended') {
			void this.audioCtx.resume();
		}
	}

	private startVisualizerLoop() {
		if (this.rafId !== null) return;
		const tick = () => {
			this.tickVisualizer();
			this.rafId = requestAnimationFrame(tick);
		};
		this.rafId = requestAnimationFrame(tick);
	}

	private tickVisualizer() {
		const analyser = this.analyser;
		const buffer = this.freqBuffer;
		if (!analyser || !buffer) return;

		if (this.isPlaying) {
			analyser.getByteFrequencyData(buffer);
			const range = VISUALIZER_BIN_END - VISUALIZER_BIN_START;
			const binsPerBar = range / VISUALIZER_BARS;
			const next = new Array<number>(VISUALIZER_BARS);
			for (let bar = 0; bar < VISUALIZER_BARS; bar++) {
				const start = VISUALIZER_BIN_START + Math.floor(bar * binsPerBar);
				const end = VISUALIZER_BIN_START + Math.floor((bar + 1) * binsPerBar);
				let sum = 0;
				for (let i = start; i < end; i++) sum += buffer[i];
				next[bar] = sum / (end - start) / 255;
			}
			this.levels = next;
		} else {
			// Ease bars to zero when paused; stop assigning once we're already at rest.
			const current = this.levels;
			if (current.some((v) => v > 0.001)) {
				this.levels = current.map((v) => v * VISUALIZER_DECAY);
			} else if (current.some((v) => v !== 0)) {
				this.levels = new Array(VISUALIZER_BARS).fill(0);
			}
		}
	}
}

export function setPlayer(): PlayerState {
	return setContext(PLAYER_KEY, new PlayerState());
}

export function getPlayer(): PlayerState {
	return getContext(PLAYER_KEY);
}
