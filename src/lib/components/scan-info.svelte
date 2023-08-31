<script lang="ts">
  import { formatRelative } from 'date-fns';
  import { clickoutside } from '@svelteuidev/composables';
  import Icon from '@iconify/svelte';
  import { scan } from '$lib/stores/scan';
  export let variant: 'ghost' | 'solid' = 'ghost';
  let open = false;
</script>

<div
  class="relative"
  use:clickoutside={{ enabled: open, callback: () => (open = false) }}
>
  <button class={variant} on:click={() => (open = !open)}
    ><Icon width={18} height={18} icon="mingcute:information-line" /></button
  >
  {#if open && $scan !== undefined}
    <div
      class="absolute bg-slate-100 rounded-xl border border-slate-400 -right-16 top-[calc(100%_+_1rem)] w-max overflow-hidden z-20 dark:bg-gray-800 dark:border-gray-950 flex flex-col"
    >
      <div class="flex flex-col py-2 gap-2">
        <h3 class="px-2 pb-1 font-bold">Server</h3>
        <h4 class="px-2">Total song count: {$scan.totalSongCount}</h4>
        <h4 class="px-2">Total album count: {$scan.totalAlbumCount}</h4>
        <div class="w-full border-b border-slate-200 dark:border-gray-700" />
        <h3 class="px-2 pb-1 font-bold">Scan</h3>
        <div class="px-2 flex flex-row gap-10">
          <h4>Last scan:</h4>
          <h4>{formatRelative(new Date($scan.lastScan), new Date())}</h4>
        </div>
        <div class="px-2 flex flex-row space-between">
          <button class="ghost mr-auto"
            ><Icon icon="mingcute:refresh-1-fill" />QUICK SCAN</button
          >
          <button class="ghost"
            ><Icon icon="mingcute:refresh-2-fill" />FULL SCAN</button
          >
        </div>
      </div>
    </div>
  {/if}
</div>
