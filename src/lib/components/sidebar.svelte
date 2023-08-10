<script lang="ts">
  import { goto } from '$app/navigation';
  import IconButton from './icon-button.svelte';
  import { user } from '$lib/stores/auth';
</script>

<aside
  class={'flex flex-col w-20 bg-secondary-bg my-4 ml-4 rounded-xl items-center py-4 justify-between'}
>
  <section class={'flex flex-col items-center gap-2'}>
    <IconButton icon="mingcute:home-4-line" on:click={() => goto('/app')} />
    <IconButton
      icon="mingcute:album-line"
      on:click={() => goto('/app/album')}
    />
  </section>
  <section class={'flex flex-col items-center gap-2'}>
    <IconButton icon="mingcute:power-line" on:click={user.signOut} />
  </section>
</aside>
