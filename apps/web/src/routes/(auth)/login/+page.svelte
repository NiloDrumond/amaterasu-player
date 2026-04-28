<script lang="ts">
	import LoginForm from './login-form.svelte';

	let { form } = $props();
</script>

<div class="flex min-h-svh flex-col items-center justify-center gap-6 bg-muted p-6 md:p-10">
	<div class="flex w-full max-w-sm flex-col gap-6">
		<a href="##" class="flex items-center gap-2 self-center font-medium">Amaterasu Player</a>
		<LoginForm {form} />
	</div>
</div>
