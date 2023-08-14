<script lang="ts">
  import { signin } from './store';
  import { user } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { signInSchema, type SignInErrors } from './data';

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
</script>

<div class="flex flex-col gap-8 items-center">
  <h1 class="text-center text-purple-500">Welcome Home</h1>
  <form on:submit|preventDefault={validate} class="space-y-5">
    <div class="flex flex-col">
      <label class="mb-1" for="username"> Username </label>
      <input name="username" type="text" bind:value={$signin.username} />
      {#if errors.username}
        <label class="mt-1 text-error" for="error">{errors.username[0]}</label>
      {/if}
    </div>
    <div class="flex flex-col">
      <label class="mb-1" for="password"> Password </label>
      <input name="password" type="password" bind:value={$signin.password} />
      {#if errors.password}
        <label class="mt-1 text-error" for="error">{errors.password[0]}</label>
      {/if}
    </div>
    <button
      disabled={loading}
      class="disabled:cursor-wait bg-purple-500 hover:bg-purple-700 w-full drop-shadow-xl"
      type="submit">Sign In</button
    >
    {#if errors.message}
      <div class="text-red-500 p-1">{errors.message}</div>
    {/if}
  </form>
</div>
