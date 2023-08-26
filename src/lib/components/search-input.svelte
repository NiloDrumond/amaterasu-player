<script lang="ts">
  import Icon from '@iconify/svelte';

  import { debounce } from 'lodash';
  import { onDestroy } from 'svelte';

  let inputValue = '';
  export let onChange: (value: string) => void;

  const debouncedInput = debounce((value: string) => {
    onChange(value);
  }, 300);

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    debouncedInput(target.value);
  }

  onDestroy(() => {
    debouncedInput.cancel();
  });
</script>

<div class="relative h-fit">
  <input
    bind:value={inputValue}
    on:input={handleInput}
    type="search"
    class="peer bg-slate-200 border border-slate-400 pl-10 placeholder-primary focus:placeholder:opacity-0 focus:border-crystal rounded-lg py-2 px-4 transition-colors focus-visible:outline-none dark:bg-gray-700 dark:border-gray-950 dark:hover:border-crystal"
    placeholder="Search..."
  />
  <div class="absolute left-3 top-3 peer-focus:text-crystal">
    <Icon icon="mingcute:search-2-line" width={18} height={18} />
  </div>
</div>
