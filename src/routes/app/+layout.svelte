<script lang="ts">
  import Sidebar from '$lib/components/Sidebar.svelte';
  import { user } from '$lib/stores/auth';

  $: if (!$user) {
    user.signOut();
  }
</script>

<div class="h-[100vh] w-[100vw] flex flex-row bg-primary-bg">
  <div class={'flex flex-row flex-1 w-full'}>
    <Sidebar />
    <main class={'flex justify-center items-center flex-1 p-4'}>
      <slot />
    </main>
  </div>
</div>
