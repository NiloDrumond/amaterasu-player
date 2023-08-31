import { ndClient } from '$lib/navidrome/client';
import type { NDSong } from '$lib/navidrome/types';
import { derived, get, writable } from 'svelte/store';
import { audioPlayer } from './audio';

type PlayerQueueStore = {
  currentSongIdx: number;
  queue: NDSong[];
  repeat: 'no' | 'all' | 'one';
};
function createPlayerQueue() {
  const store = writable<PlayerQueueStore>({
    queue: [],
    currentSongIdx: -1,
    repeat: 'no',
  });
  const { subscribe, update } = store;

  function setQueue(queue: NDSong[], songIdx?: number) {
    update((prev) => ({
      ...prev,
      currentSongIdx: songIdx === undefined ? 0 : songIdx,
      queue: queue,
    }));
    audioPlayer.onChangeSong();
    audioPlayer.startPlaying();
  }

  function addToQueue(song: NDSong, position: 'next' | 'later') {
    if (position === 'later') {
      update((prev) => ({ ...prev, queue: [...prev.queue, song] }));
    }
    if (position === 'next') {
      update((prev) => {
        return {
          ...prev,
          queue: [
            ...prev.queue.slice(0, prev.currentSongIdx + 1),
            song,
            ...prev.queue.slice(prev.currentSongIdx + 1),
          ],
        };
      });
    }
  }

  function selectSong(idx: number) {
    const different = get(store).currentSongIdx !== idx;
    if (different) {
      update((prev) => ({ ...prev, currentSongIdx: idx }));
      audioPlayer.onChangeSong();
    }
  }

  function nextSong(): NDSong | undefined {
    let changed = false;
    update((prev) => {
      if (prev.currentSongIdx + 1 >= prev.queue.length) {
        return prev;
      }
      changed = true;
      audioPlayer.onChangeSong();
      return { ...prev, currentSongIdx: prev.currentSongIdx + 1 };
    });
    if (!changed) return undefined;
    return get(store).queue[get(store).currentSongIdx];
  }

  function previousSong() {
    update((prev) => {
      if (prev.currentSongIdx === 0) {
        return prev;
      }
      audioPlayer.onChangeSong();
      return { ...prev, currentSongIdx: prev.currentSongIdx - 1 };
    });
  }

  function clearQueue() {
    update((prev) => ({ ...prev, currentSongIdx: -1, queue: [] }));
    audioPlayer.update((prev) => ({ ...prev, paused: true }));
  }

  function reorderQueue(queue: NDSong[]) {
    const currentSongId = get(store).queue[get(store).currentSongIdx].id;
    const newIdx = queue.findIndex((song) => song.id === currentSongId);
    update((prev) => ({ ...prev, queue, currentSongIdx: newIdx }));
  }

  return {
    addToQueue,
    subscribe,
    setQueue,
    nextSong,
    previousSong,
    clearQueue,
    reorderQueue,
    selectSong,
  };
}

export const playerQueue = createPlayerQueue();
export const currentSong = derived(playerQueue, ($playerQueue) => {
  if (
    $playerQueue.currentSongIdx < 0 ||
    $playerQueue.currentSongIdx >= $playerQueue.queue.length
  )
    return undefined;
  return $playerQueue.queue[$playerQueue.currentSongIdx];
});

export const currentStreamUrl = derived(currentSong, ($currentSong) => {
  if (!$currentSong) return undefined;
  return ndClient.getSongStreamUrl({
    songId: $currentSong.id,
  });
});