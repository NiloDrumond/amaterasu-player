<script lang="ts">
  import Sidebar from '$lib/components/sidebar.svelte';
  import { user } from '$lib/stores/auth';

  $: if (!$user) {
    user.signOut();
  }
</script>

<div
  class="h-[100vh] w-[100vw] flex flex-row max-w-[100vw] max-h-[100vh] overflow-hidden"
>
  <Sidebar />
  <div id="scrollableContainer" class="m-4 w-full overflow-y-auto max-h-full">
    <main class={'flex justify-center items-center flex-1 max-w-full '}>
      <slot />
    </main>
  </div>
</div>
