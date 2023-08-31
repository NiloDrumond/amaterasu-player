<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<aside
  class={'flex flex-col items-start border-r border-slate-300 shadow-md min-w-[10rem] dark:border-gray-950 dark:bg-gray-900 dark:shadow-none'}
>
  <section
    class={'flex flex-col items-center w-full border-b border-slate-200 dark:border-gray-700'}
  >
    <!-- <a href="/app" class="w-full"> -->
    <!--   <button class="menu-item p-4"> -->
    <!--     <Icon icon="mingcute:home-4-line" width={20} height={20} />Home -->
    <!--   </button> -->
    <!-- </a> -->
    <a href="/app/album" class="w-full">
      <button class="menu-item p-4">
        <Icon icon="mingcute:album-line" width={20} height={20} />Albums
      </button>
    </a>
    <a href="/app/song" class="w-full">
      <button class="menu-item p-4">
        <Icon icon="mingcute:music-line" width={20} height={20} />Songs
      </button>
    </a>
  </section>
  <section class="flex flex-col w-full">
    <a href="/app/playlist" class="w-full">
      <button class="menu-item p-4">
        <Icon icon="mingcute:playlist-2-fill" width={20} height={20} />Playlists
      </button>
    </a>
  </section>
</aside>
