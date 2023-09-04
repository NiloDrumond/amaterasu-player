import { writable } from 'svelte/store';

type PaginationState = {
  page: number;
  pageSize: number;
  totalAmount: number;
  hasNextPage: boolean;
  hasPrevPage: boolean;
};
export function createPaginationStore(
  initialState: Omit<PaginationState, 'hasNextPage' | 'hasPrevPage'>,
) {
  const store = writable<PaginationState>({
    ...initialState,
    hasPrevPage: initialState.page > 0,
    hasNextPage: initialState.totalAmount > initialState.pageSize,
  });
  const { update, set } = store;

  function nextPage() {
    update((prev) => {
      const newPage = prev.page + 1;
      return {
        ...prev,
        page: newPage,
        hasNextPage: initialState.totalAmount > (newPage + 1) * prev.pageSize,
        hasPrevPage: true,
      };
    });
  }

  function prevPage() {
    update((prev) => {
      const newPage = prev.page - 1;
      return {
        ...prev,
        page: newPage,
        hasPrevPage: newPage > 0,
        hasNextPage: true,
      };
    });
  }

  function setPage(page: number) {
    set({
      ...initialState,
      page,
      hasPrevPage: page !== 0,
      hasNextPage: initialState.totalAmount > initialState.pageSize,
    });
  }

  return { ...store, nextPage, prevPage, setPage };
}
