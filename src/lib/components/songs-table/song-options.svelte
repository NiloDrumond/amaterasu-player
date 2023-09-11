<script lang="ts">
  import Icon from '@iconify/svelte';
  import { getSongMenuContext } from '../song-menu.svelte';

  const { toggle } = getSongMenuContext();
</script>

<button
  class="ghost !p-1"
  on:click={getSongMenuContext()}
  on:click={(e) => {
    toggle();
    e.stopPropagation();
    e.preventDefault();
  }}
>
  <Icon icon="mingcute:more-1-fill" width={20} height={20} />
</button>
