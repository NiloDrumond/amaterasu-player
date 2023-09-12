<script lang="ts">
  import Icon from '@iconify/svelte';
  import Playlists from './playlists.svelte';
</script>

<aside
  class={'flex flex-col items-start border-r border-slate-300 shadow-md min-w-[10rem] dark:border-gray-950 dark:bg-gray-900 dark:shadow-none'}
>
  <section
    class={'flex flex-col items-center w-full border-b border-slate-200 dark:border-gray-700'}
  >
    <a href="/app/album" class="w-full">
      <button class="menu-item">
        <Icon icon="mingcute:album-line" width={20} height={20} />Albums
      </button>
    </a>
    <a href="/app/song" class="w-full">
      <button class="menu-item">
        <Icon icon="mingcute:music-line" width={20} height={20} />Songs
      </button>
    </a>
  </section>
  <Playlists />
</aside>
