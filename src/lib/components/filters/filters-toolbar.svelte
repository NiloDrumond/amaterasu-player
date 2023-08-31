<script lang="ts">
  import Icon from '@iconify/svelte';
  import SearchInput from '../search-input.svelte';

  export let nextPage: undefined | (() => void) = undefined;
  export let prevPage: undefined | (() => void) = undefined;
  export let page: number = 0;
  export let onSearch: (value: string) => void;
</script>

<div
  class="mb-2 px-3 py-2 flex flex-row sticky top-0 border border-slate-300 bg-slate-100 shadow-lg rounded-xl items-center dark:bg-gray-900 dark:border-gray-950 gap-2 z-10"
>
  <SearchInput onChange={onSearch} />
  <button class="ghost">
    <Icon icon="mingcute:shuffle-2-line" />
    RANDOM
  </button>
  <button class="ghost">
    <Icon icon="mingcute:flame-line" />
    POPULAR
  </button>
  <div class="flex-1" />
  <button class="ghost">
    <Icon icon="mingcute:filter-2-line" />
    FILTERS
  </button>
  {#if prevPage}
    <button class="solid" on:click={prevPage}>
      <Icon icon="mingcute:left-fill" />
    </button>
  {/if}
  {#if page > 0}
    <p>{page + 1}</p>
  {/if}
  {#if nextPage}
    <button class="solid" on:click={nextPage}>
      <Icon icon="mingcute:right-fill" />
    </button>
  {/if}
</div>
