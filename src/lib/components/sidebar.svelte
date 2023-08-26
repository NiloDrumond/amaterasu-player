<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<aside
  class={'flex flex-col bg-secondary-bg items-start border-r border-slate-300 shadow-md min-w-[8rem] dark:border-gray-950 dark:bg-gray-900 dark:shadow-none'}
>
  <section class={'flex flex-col items-center w-full'}>
    <a href="/app/album" class="w-full">
      <button class="menu-item p-4">
        <Icon icon="mingcute:album-line" width={20} height={20} />Albums
      </button>
    </a>

    <a href="/app/album" class="w-full">
      <button class="menu-item p-4">
        <Icon icon="mingcute:music-line" width={20} height={20} />Songs
      </button>
    </a>
  </section>
</aside>
