<script lang="ts">
  import { signin } from './store';
  import { enhance } from '$app/forms';
  import type { SignInOutput } from './data';
  import { user } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let loading = false;
  export let form: SignInOutput | null;
  $: if ($signin.username) {
    if (form?.success === false && form?.errors) {
      form.errors.username = undefined;
    }
  }
  $: if ($signin.password) {
    if (form?.success === false && form?.errors) {
      form.errors.password = undefined;
    }
  }
  $: if (form?.success) {
    user.setUser(form.user);
    goto('/app');
  }
  $: if (user) {
    goto('/app');
  }
</script>

<div class="flex flex-col gap-8 items-center">
  <h1 class="text-center text-purple-500">Welcome Home</h1>
  <!-- {#if form?.incorrect} -->
  <!-- 	<p class="mt-3 text-red-500 text-center font-semibold">Invalid user/password</p> -->
  <!-- {/if} -->

  <form
    method="POST"
    class="space-y-5"
    use:enhance={async ({ formData, action }) => {
      loading = true;
      const response = await fetch(action, {
        method: 'POST',
        body: formData,
      });
      const responseData = await response.json();
      form = responseData;
      loading = false;
      return async ({ update }) => {
        update();
      };
    }}
  >
    <div class="flex flex-col">
      <label class="mb-1" for="username"> Username </label>
      <input name="username" type="text" bind:value={$signin.username} />
      {#if form?.success === false && form?.errors?.username}
        <label class="mt-1 text-error" for="error"
          >{form.errors.username[0]}</label
        >
      {/if}
    </div>
    <div class="flex flex-col">
      <label class="mb-1" for="password"> Password </label>
      <input name="password" type="password" bind:value={$signin.password} />
      {#if form?.success === false && form?.errors?.password}
        <label class="mt-1 text-error" for="error"
          >{form.errors.password[0]}</label
        >
      {/if}
    </div>
    <button
      disabled={loading}
      class="disabled:cursor-wait bg-purple-500 hover:bg-purple-700 w-full drop-shadow-xl"
      type="submit">Sign In</button
    >
    {#if form?.success === false && form?.message}
      <div class="text-red-500 p-1">{form.message}</div>
    {/if}
  </form>
</div>
