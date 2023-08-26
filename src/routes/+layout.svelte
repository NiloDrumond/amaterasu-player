<script>
  import { user } from '$lib/stores/auth';
  import '../app.css';
  import '@fontsource/inter/400.css';
  import '@fontsource/inter/600.css';
  import '@fontsource/inter/700.css';
  import '@fontsource/arvo/400.css';
  import '@fontsource/arvo/700.css';

  $: user.loadUser();
</script>

<div
  class="h-[100vh] w-[100vw] flex items-center justify-center flex-col relative bg-slate-100"
>
  <slot />
</div>
