import {
  NDSongListSort,
  type NDSongListParams,
  NDSortOrder,
} from '$lib/navidrome/types';
import { createPaginationStore } from '$lib/stores/pagination';
import { scan } from '$lib/stores/scan';
import { get, writable } from 'svelte/store';

const POPULAR_SORT = {
  _sort: NDSongListSort.PLAY_COUNT,
  _order: NDSortOrder.DESC,
};
const RANDOM_SORT = {
  _sort: NDSongListSort.RANDOM,
  _order: NDSortOrder.ASC,
};
const ALPHABETICAL_SORT = {
  _sort: NDSongListSort.TITLE,
  _order: NDSortOrder.ASC,
};
const PAGE_SIZE = 50;
function createSongListParams() {
  const store = writable<NDSongListParams>({
    _end: PAGE_SIZE,
    ...ALPHABETICAL_SORT,
  });
  const { subscribe, update } = store;
  const pagination = createPaginationStore({
    page: 0,
    pageSize: PAGE_SIZE,
    totalAmount: get(scan)?.totalSongCount || 0,
  });

  function setTitle(value: string) {
    let title: string | undefined = value;
    if (value.trim().length === 0) {
      title = undefined;
    }
    update((prev) => ({ ...prev, title }));
  }

  function setSortAlphabetical() {
    update((prev) => ({
      ...prev,
      ...ALPHABETICAL_SORT,
    }));
    pagination.setPage(0);
  }

  function setSortRandom() {
    const state = get(store);
    const current = state._sort;
    if (current === NDSongListSort.RANDOM) {
      setSortAlphabetical();
    } else {
      update((prev) => ({
        ...prev,
        ...RANDOM_SORT,
      }));
      pagination.setPage(0);
    }
  }

  function setSortPopular() {
    const state = get(store);
    const current = state._sort;
    if (current === NDSongListSort.PLAY_COUNT) {
      setSortAlphabetical();
    } else {
      update((prev) => ({
        ...prev,
        ...POPULAR_SORT,
      }));
      pagination.setPage(0);
    }
  }

  pagination.subscribe((state) => {
    const _start = state.page * PAGE_SIZE;
    const _end = (state.page + 1) * PAGE_SIZE;
    update((prev) => ({ ...prev, _start, _end }));
  });
  return {
    subscribe,
    setTitle,
    setSortRandom,
    setSortPopular,
    pagination,
  };
}

export const songListParams = createSongListParams();
