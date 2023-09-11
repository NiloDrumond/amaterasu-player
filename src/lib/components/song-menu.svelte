<script lang="ts" context="module">
  export const [getSongMenuContext, createSongMenuContext] = defineService<{
    open: boolean;
    toggle: () => void;
  }>('song-menu');
</script>

<script lang="ts">
  import type { NDSong } from '$lib/navidrome/types';
  import { defineService } from '$lib/utils/service-helper';
  import Icon from '@iconify/svelte';
  import { clickoutside } from '@svelteuidev/composables';

  export let song: NDSong;

  let open = false;

  const { toggle } = createSongMenuContext({
    open,
    toggle: () => {
      open = !open;
    },
  });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
{#if open}
  <div
    class="absolute w-[100vw] h-[100vh] cursor-default left-0 top-0 z-10"
    on:click={() => {
      open = false;
    }}
  />
{/if}
<div
  class="relative"
  use:clickoutside={{ enabled: open, callback: () => (open = false) }}
>
  <slot />
  {#if open}
    <div
      class="absolute bg-slate-100 rounded-xl border border-slate-400 w-max -right-4 top-[calc(100%_+_1rem)] overflow-hidden z-20 dark:bg-gray-800 dark:border-gray-950"
    >
      <button
        on:click={(e) => {
          e.preventDefault();
          e.stopPropagation();
          // TODO: add to playlist
          toggle();
        }}
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        ><Icon width={18} height={18} icon="ph:music-notes-plus-bold" />Add to
        Playlist</button
      >
      <button
        on:click={(e) => {
          e.preventDefault();
          e.stopPropagation();
          // TODO: play next
          toggle();
        }}
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        ><Icon width={18} height={18} icon="iconamoon:playlist" />Play Next</button
      >
      <button
        on:click={(e) => {
          e.preventDefault();
          e.stopPropagation();
          // TODO: play later
          toggle();
        }}
        class="menu-item border-t border-slate-300 dark:border-gray-950"
        ><Icon width={18} height={18} icon="ph:list-plus-bold" />Play Later</button
      >
    </div>
  {/if}
</div>
