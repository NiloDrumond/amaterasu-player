<script lang="ts">
  import { signin } from './store';
  import { user } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { signInSchema, type SignInErrors } from './data';
  import { PUBLIC_WALLPAPER_COUNT } from '$env/static/public';
  import ThemePicker from '$lib/components/theme-picker.svelte';

  let loading = false;
  let errors: SignInErrors = {};
  async function validate() {
    const signInData = signInSchema.safeParse($signin);
    if (signInData.success) {
      try {
        user.authenticate(signInData.data);
      } catch (e) {
        if (e instanceof Error) {
          errors.message = e.message;
        }
      }
      signin.set({});
      return;
    } else {
      errors = signInData.error.formErrors.fieldErrors;
    }
  }
  $: if ($signin.username && errors.username) {
    errors.username = undefined;
  }
  $: if ($signin.password && errors.password) {
    errors.password = undefined;
  }
  $: if ($user) {
    goto('/app');
  }
  function getRandomWallpaper(): string {
    let count;
    try {
      count = parseInt(PUBLIC_WALLPAPER_COUNT);
    } catch (error) {
      count = 1;
    }
    const index = Math.floor(Math.random() * count) + 1;
    return `bg-wallpaper-${index}`;
  }
  const wallpaper = getRandomWallpaper();
</script>

<div
  class={`flex flex-col items-center justify-center w-full h-full bg-cover ${wallpaper} relative`}
>
  <div class="absolute top-0 right-0 p-8">
    <ThemePicker variant="solid" />
  </div>
  <div
    class="flex flex-col gap-8 items-center bg-slate-200 bg-opacity-50 p-8 rounded-xl border-2 border-slate-50 shadow-md dark:bg-gray-800 dark:bg-opacity-80"
  >
    <h1 class="text-center text-crystal">Welcome Home</h1>
    <form on:submit|preventDefault={validate} class="space-y-5">
      <div class="flex flex-col group">
        <label
          class="mb-1 group-focus-within:text-crystal transition-all"
          for="username"
        >
          Username
        </label>
        <input
          name="username"
          type="text"
          bind:value={$signin.username}
          class="minimal"
        />
        {#if errors.username}
          <label class="mt-1 text-red-600 dark:text-red-400" for="error"
            >{errors.username[0]}</label
          >
        {/if}
      </div>
      <div class="flex flex-col group">
        <label
          class="mb-1 group-focus-within:text-crystal transition-all"
          for="password"
        >
          Password
        </label>
        <input
          name="password"
          type="password"
          bind:value={$signin.password}
          class="minimal"
        />
        {#if errors.password}
          <label class="mt-1 text-red-600 dark:text-red-400" for="error"
            >{errors.password[0]}</label
          >
        {/if}
      </div>
      <button
        disabled={loading}
        class="p-3 rounded-lg disabled:cursor-wait bg-crystal text-slate-100 hover:text-slate-50 hover:bg-opacity-80 w-full shadow-md dark:text-gray-800"
        type="submit">Sign In</button
      >
      {#if errors.message}
        <div class="text-red-600 dark:text-red-400 p-1">{errors.message}</div>
      {/if}
    </form>
  </div>
</div>
