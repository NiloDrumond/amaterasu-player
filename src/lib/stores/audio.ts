import { writable } from 'svelte/store';

type AudioPlayerState = {
  duration: number;
  currentTime: number;
  paused: boolean;
  volume: number;
  muted: boolean;
};
const STORAGE_KEY = 'audio-player';
function createAudioPlayer() {
  const { subscribe, set, update } = writable<AudioPlayerState>({
    duration: 0,
    currentTime: 0,
    paused: true,
    volume: 0.5,
    muted: false,
  });

  function onChangeSong() {
    update((prev) => ({ ...prev, currentTime: 0 }));
  }

  function startPlaying() {
    update((prev) => ({ ...prev, paused: false }));
  }

  function loadState() {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      const parsed = JSON.parse(raw) as AudioPlayerState;
      set({ ...parsed, paused: true });
    }
  }

  function togglePlaying() {
    update((prev) => ({ ...prev, paused: !prev.paused }));
  }

  function toggleMute() {
    update((prev) => ({ ...prev, muted: !prev.muted }));
  }

  loadState();
  return {
    subscribe,
    set,
    update,
    onChangeSong,
    startPlaying,
    togglePlaying,
    toggleMute,
  };
}

export const audioPlayer = createAudioPlayer();
audioPlayer.subscribe((state) => {
  if (state) {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  }
});
