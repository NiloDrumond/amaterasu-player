<script lang="ts">
  import Icon from '@iconify/svelte';
  import { selected } from './store';

  function handleDeselectAll() {
    selected.set([]);
  }
</script>

<div
  class={'sticky h-0 z-10 top-20  w-full  left-0 flex flex-row gap-2 items-center text-slate-800'}
>
  <div class="relative w-full">
    <div
      class="absolute h-20 -top-20 w-full flex flex-row items-center bg-crystal rounded-tl-lg rounded-tr-lg py-4 px-2 gap-2"
    >
      <button on:click={handleDeselectAll} class="relative group">
        <Icon icon="ph:x" width={20} height={20} />
        <span
          class="absolute left-3 w-5 h-[1px] bottom-[10px] bg-black transform scale-x-0 group-hover:scale-x-100"
        ></span>
      </button>
      <p>{$selected.length} items selected</p>
      <div class="flex-1" />
      <button class="hover:underline">
        <Icon icon="mingcute:play-fill" />
        PLAY NOW
      </button>
      <button class="hover:underline">
        <Icon icon="iconamoon:playlist" />
        PLAY NEXT
      </button>
      <button class="hover:underline">
        <Icon icon="ph:list-plus-bold" />
        PLAY LATER
      </button>
      <button class="hover:underline">
        <Icon icon="ph:music-notes-plus-bold" />
        ADD TO PLAYLIST
      </button>
    </div>
  </div>
</div>
