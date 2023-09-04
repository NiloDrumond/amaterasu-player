import { ndClient } from '$lib/navidrome/client';
import { writable } from 'svelte/store';

type ScanStore = {
  totalSongCount: number;
  totalAlbumCount: number;
  lastScan: Date;
};
function createScan() {
  const store = writable<ScanStore | undefined>();
  const { subscribe, set } = store;

  async function fetchStatus() {
    const response = await ndClient.getScanStatus();
    const {
      scanStatus: { count, folderCount, lastScan },
    } = response['subsonic-response'];
    set({ totalAlbumCount: folderCount, totalSongCount: count, lastScan });
  }

  function loadStatus(): void {
    const scanStatus = localStorage.getItem('scanStatus');
    if (scanStatus) {
      const parsed = JSON.parse(scanStatus) as ScanStore;
      set(parsed);
    }
  }

  return { subscribe, loadStatus, fetchStatus };
}

export const scan = createScan();
