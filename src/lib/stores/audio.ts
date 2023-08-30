import { writable } from 'svelte/store';

type AudioPlayerStore = {
  duration: number;
  currentTime: number;
  paused: boolean;
  volume: number;
  muted: boolean;
};
function createAudioPlayer() {
  const { subscribe, set, update } = writable<AudioPlayerStore>({
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

  return { subscribe, set, update, onChangeSong, startPlaying };
}

export const audioPlayer = createAudioPlayer();
